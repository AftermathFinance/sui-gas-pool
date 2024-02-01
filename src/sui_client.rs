// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::types::GasCoin;
use crate::{retry_forever, retry_with_max_delay};
use futures_util::stream::FuturesUnordered;
use futures_util::StreamExt;
use itertools::Itertools;
use std::collections::HashMap;
use std::time::Duration;
use sui_json_rpc_types::SuiTransactionBlockEffectsAPI;
use sui_json_rpc_types::{
    SuiData, SuiObjectDataOptions, SuiObjectResponse, SuiTransactionBlockEffects,
    SuiTransactionBlockResponseOptions,
};
use sui_sdk::SuiClientBuilder;
use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::coin::{PAY_MODULE_NAME, PAY_SPLIT_N_FUNC_NAME};
use sui_types::gas_coin::GAS;
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{
    Argument, ObjectArg, ProgrammableTransaction, Transaction, TransactionKind,
};
use sui_types::SUI_FRAMEWORK_PACKAGE_ID;
use tap::TapFallible;
use tokio_retry::strategy::ExponentialBackoff;
#[cfg(not(test))]
use tokio_retry::strategy::FixedInterval;
use tokio_retry::Retry;
use tracing::{debug, info};

#[derive(Clone)]
pub struct SuiClient {
    sui_client: sui_sdk::SuiClient,
}

impl SuiClient {
    pub async fn new(fullnode_url: &str) -> Self {
        let sui_client = SuiClientBuilder::default()
            .max_concurrent_requests(100000)
            .build(fullnode_url)
            .await
            .unwrap();
        Self { sui_client }
    }

    pub async fn get_all_owned_sui_coins(&self, address: SuiAddress) -> Vec<GasCoin> {
        info!(
            "Querying all gas coins owned by sponsor address: {}",
            address
        );
        let mut cursor = None;
        let mut coins = Vec::new();
        loop {
            let page = retry_forever!(async {
                self.sui_client
                    .coin_read_api()
                    .get_coins(address, None, cursor, None)
                    .await
                    .tap_err(|err| debug!("Failed to get owned gas coins: {:?}", err))
            })
            .unwrap();
            for coin in page.data {
                coins.push(GasCoin {
                    object_ref: coin.object_ref(),
                    balance: coin.balance,
                });
            }
            if page.has_next_page {
                cursor = page.next_cursor;
            } else {
                break;
            }
        }
        coins
    }

    pub async fn get_reference_gas_price(&self) -> u64 {
        retry_forever!(async {
            self.sui_client
                .governance_api()
                .get_reference_gas_price()
                .await
                .tap_err(|err| debug!("Failed to get reference gas price: {:?}", err))
        })
        .unwrap()
    }

    pub async fn get_latest_gas_objects(
        &self,
        object_ids: impl IntoIterator<Item = ObjectID>,
    ) -> HashMap<ObjectID, Option<GasCoin>> {
        let tasks: FuturesUnordered<_> = object_ids
            .into_iter()
            .chunks(50)
            .into_iter()
            .map(|chunk| {
                let chunk: Vec<_> = chunk.collect();
                let sui_client = self.sui_client.clone();
                tokio::spawn(async move {
                    retry_forever!(async {
                        let chunk = chunk.clone();
                        let result = sui_client
                            .clone()
                            .read_api()
                            .multi_get_object_with_options(
                                chunk.clone(),
                                SuiObjectDataOptions::default().with_bcs(),
                            )
                            .await
                            .map_err(anyhow::Error::from)?;
                        if result.len() != chunk.len() {
                            anyhow::bail!(
                                "Unable to get all gas coins, got {} out of {}",
                                result.len(),
                                chunk.len()
                            );
                        }
                        Ok(chunk.into_iter().zip(result).collect::<Vec<_>>())
                    })
                    .unwrap()
                })
            })
            .collect();
        let objects: Vec<_> = tasks.collect().await;
        let objects: Vec<_> = objects.into_iter().flat_map(|r| r.unwrap()).collect();
        objects
            .into_iter()
            .map(|(id, response)| {
                let object = match Self::try_get_sui_coin_balance(&response) {
                    Some(coin) => {
                        debug!("Got updated gas coin info: {:?}", coin);
                        Some(coin)
                    }
                    None => {
                        debug!("Object no longer exists: {:?}", id);
                        None
                    }
                };
                (id, object)
            })
            .collect()
    }

    pub fn construct_coin_split_pt(
        gas_coin: Argument,
        split_count: u64,
    ) -> ProgrammableTransaction {
        let mut pt_builder = ProgrammableTransactionBuilder::new();
        let pure_arg = pt_builder.pure(split_count).unwrap();
        pt_builder.programmable_move_call(
            SUI_FRAMEWORK_PACKAGE_ID,
            PAY_MODULE_NAME.into(),
            PAY_SPLIT_N_FUNC_NAME.into(),
            vec![GAS::type_tag()],
            vec![gas_coin, pure_arg],
        );
        pt_builder.finish()
    }

    pub async fn calibrate_gas_cost_per_object(
        &self,
        sponsor_address: SuiAddress,
        gas_coin: &GasCoin,
    ) -> u64 {
        const SPLIT_COUNT: u64 = 500;
        let mut pt_builder = ProgrammableTransactionBuilder::new();
        let object_arg = pt_builder
            .obj(ObjectArg::ImmOrOwnedObject(gas_coin.object_ref))
            .unwrap();
        let pure_arg = pt_builder.pure(SPLIT_COUNT).unwrap();
        pt_builder.programmable_move_call(
            SUI_FRAMEWORK_PACKAGE_ID,
            PAY_MODULE_NAME.into(),
            PAY_SPLIT_N_FUNC_NAME.into(),
            vec![GAS::type_tag()],
            vec![object_arg, pure_arg],
        );
        let pt = pt_builder.finish();
        let response = retry_forever!(async {
            self.sui_client
                .read_api()
                .dev_inspect_transaction_block(
                    sponsor_address,
                    TransactionKind::ProgrammableTransaction(pt.clone()),
                    None,
                    None,
                    None,
                )
                .await
        })
        .unwrap();
        let gas_used = response.effects.gas_cost_summary().gas_used();
        // Multiply by 2 to be conservative and resilient to precision loss.
        gas_used / SPLIT_COUNT * 2
    }

    pub async fn execute_transaction(
        &self,
        tx: Transaction,
        max_delay: Duration,
    ) -> anyhow::Result<SuiTransactionBlockEffects> {
        let digest = *tx.digest();
        debug!(?digest, "Executing transaction: {:?}", tx);
        let response = retry_with_max_delay!(
            async {
                self.sui_client
                    .quorum_driver_api()
                    .execute_transaction_block(
                        tx.clone(),
                        SuiTransactionBlockResponseOptions::full_content(),
                        None,
                    )
                    .await
                    .tap_err(|err| debug!(?digest, "execute_transaction error: {:?}", err))
                    .map_err(anyhow::Error::from)
                    .and_then(|r| r.effects.ok_or_else(|| anyhow::anyhow!("No effects")))
            },
            max_delay
        );
        debug!(?digest, "Transaction execution response: {:?}", response);
        response
    }

    fn try_get_sui_coin_balance(object: &SuiObjectResponse) -> Option<GasCoin> {
        let data = object.data.as_ref()?;
        let object_ref = data.object_ref();
        let move_obj = data.bcs.as_ref()?.try_as_move()?;
        if move_obj.type_ != sui_types::gas_coin::GasCoin::type_() {
            return None;
        }
        let gas_coin: sui_types::gas_coin::GasCoin = bcs::from_bytes(&move_obj.bcs_bytes).ok()?;
        Some(GasCoin {
            object_ref,
            balance: gas_coin.value(),
        })
    }
}
