# Tutorial

Make sure you read the [Main README] first to get a high-level idea of what this service does.

The example `compose.yml` launches a gas pool for sponsor address `0x384b6ab10c9ec5abc54b75d8fe90f1700e3d55053c48d8980e978821303ac7d1`

You can fund the sponsor address by calling `./faucet.sh <ADDRESS>` with the address above.

You can test reserving gas coins by calling `./reserve.sh <BUDGET>`, where the budget value is in MIST (each MIST is equivalent to 1e-9 SUI).

## Example

Upon calling `./reserve.sh 1000000000` (1 SUI), you'll receive a JSON in the response body like the following.
```json
{
  "result": {
    "sponsor_address": "0x384b6ab10c9ec5abc54b75d8fe90f1700e3d55053c48d8980e978821303ac7d1",
    "reservation_id": 1,
    "gas_coins": [
      {
        "objectId": "0x00522ef2ca8b295b2025368fd7a6a08bb4ef69c25d9a7562a5efb4ef6c0d2241",
        "version": 3,
        "digest": "HBLxUCnbAs1pzV78QwePYheSyFo8xiUd9gS3sWx6bV9V"
      },
      {
        "objectId": "0x005ea99749895f7b7e4182f219cf027955e3c143eb02ebe133649a8706d7c124",
        "version": 3,
        "digest": "AinhweRTLEvmGraCQhTYBh2WnpRA8iVxibn39qt14fgS"
      },
      {
        "objectId": "0x0061c6a045715742dcc0b9244bf4e8e0b2beeaa0f8314de5da204db6a7dcab87",
        "version": 3,
        "digest": "HbdUSAyYn5ZgWfnJ3LgJeu37tF5anoztM21tMJVtr2wG"
      },
      {
        "objectId": "0x0074e9eb63f38cd5176e7f146733f0373d80ac9955c1b5cb0d66d90e468f4602",
        "version": 3,
        "digest": "HdgyWEagnprLBFeNxYDjixH3ehrzh1YzkuffTenrcLrX"
      },
      {
        "objectId": "0x00917557eaccad6dc7a58932d57c6295289ba1776a70fd7f74b18f613b61ac93",
        "version": 3,
        "digest": "ARUpkCFChnvedpGi54g8qcP2q3XD81HoyPseJZqAPZSw"
      },
      {
        "objectId": "0x0110810aebf1179ffd59a2bca5cd984f4de4f67cd7af30cb3c511191579e8c61",
        "version": 3,
        "digest": "6qic7aP6K4b25WRdPoaCHEDxYEQWfgTg85youkWUeQxu"
      },
      {
        "objectId": "0x012946a295071f1aa82d09ab80dc38307596f4dc0b2e68d198be23fb407a5f15",
        "version": 3,
        "digest": "2pUdsN2LMzo9sXVRe9BTCiwMkbBfdVSxpcmGcGGwV8fP"
      },
      {
        "objectId": "0x014a245714f06e9c8ceef0f0e4fecae3a6f1ad4d6b5da2da9f9aaf0ad32ed5da",
        "version": 3,
        "digest": "CJJsMYcaVbCmQoFYjtbe94RrFVXmS7kb9ZYsrrMhUkMN"
      },
      {
        "objectId": "0x0154bb7d10b5edbc5be81513e339d16fe8de09f998eab0513ae51191c11abfec",
        "version": 3,
        "digest": "23eCmW5KjGWdZb2BTZ3XBwX7Hv8MGPfQ74qFAfHTpPV2"
      },
      {
        "objectId": "0x016e5cb2cad01fe303be3ca5c82ce2a220797d3bb29b5e036aac0e799c1e6c5f",
        "version": 3,
        "digest": "ECAfi8RVDsmCi3vvTBhEWEYidsVarvafJTUdeY2sMYxz"
      }
    ]
  },
  "error": null
}
```

<details>
<summary>Gas pool logs</summary>

```
2025-03-25T20:37:37.285429Z DEBUG sui_gas_station::rpc::server: Received v1 reserve_gas request: ReserveGasRequest { gas_budget: 1000000000, reserve_duration_secs: 20 }
2025-03-25T20:37:37.286302Z  INFO sui_gas_station::rpc::server: Reserved gas coins with sponsor=0x384b6ab10c9ec5abc54b75d8fe90f1700e3d55053c48d8980e978821303ac7d1, budget=1000000000 and duration=20: [(0x00522ef2ca8b295b2025368fd7a6a08bb4ef69c25d9a7562a5efb4ef6c0d2241, SequenceNumber(3), o#HBLxUCnbAs1pzV78QwePYheSyFo8xiUd9gS3sWx6bV9V), (0x005ea99749895f7b7e4182f219cf027955e3c143eb02ebe133649a8706d7c124, SequenceNumber(3), o#AinhweRTLEvmGraCQhTYBh2WnpRA8iVxibn39qt14fgS), (0x0061c6a045715742dcc0b9244bf4e8e0b2beeaa0f8314de5da204db6a7dcab87, SequenceNumber(3), o#HbdUSAyYn5ZgWfnJ3LgJeu37tF5anoztM21tMJVtr2wG), (0x0074e9eb63f38cd5176e7f146733f0373d80ac9955c1b5cb0d66d90e468f4602, SequenceNumber(3), o#HdgyWEagnprLBFeNxYDjixH3ehrzh1YzkuffTenrcLrX), (0x00917557eaccad6dc7a58932d57c6295289ba1776a70fd7f74b18f613b61ac93, SequenceNumber(3), o#ARUpkCFChnvedpGi54g8qcP2q3XD81HoyPseJZqAPZSw), (0x0110810aebf1179ffd59a2bca5cd984f4de4f67cd7af30cb3c511191579e8c61, SequenceNumber(3), o#6qic7aP6K4b25WRdPoaCHEDxYEQWfgTg85youkWUeQxu), (0x012946a295071f1aa82d09ab80dc38307596f4dc0b2e68d198be23fb407a5f15, SequenceNumber(3), o#2pUdsN2LMzo9sXVRe9BTCiwMkbBfdVSxpcmGcGGwV8fP), (0x014a245714f06e9c8ceef0f0e4fecae3a6f1ad4d6b5da2da9f9aaf0ad32ed5da, SequenceNumber(3), o#CJJsMYcaVbCmQoFYjtbe94RrFVXmS7kb9ZYsrrMhUkMN), (0x0154bb7d10b5edbc5be81513e339d16fe8de09f998eab0513ae51191c11abfec, SequenceNumber(3), o#23eCmW5KjGWdZb2BTZ3XBwX7Hv8MGPfQ74qFAfHTpPV2), (0x016e5cb2cad01fe303be3ca5c82ce2a220797d3bb29b5e036aac0e799c1e6c5f, SequenceNumber(3), o#ECAfi8RVDsmCi3vvTBhEWEYidsVarvafJTUdeY2sMYxz)] reservation_id=1
```

</details>

If you don't use the gas coins within the `reserve_duration_secs` window declared in the request, the coins will be released back to the pool.

<details>
<summary>Gas pool logs</summary>

```
2025-03-25T20:37:57.398263Z DEBUG sui_gas_station::gas_pool::gas_pool_core: Coins that are expired: [0x00522ef2ca8b295b2025368fd7a6a08bb4ef69c25d9a7562a5efb4ef6c0d2241, 0x005ea99749895f7b7e4182f219cf027955e3c143eb02ebe133649a8706d7c124, 0x0061c6a045715742dcc0b9244bf4e8e0b2beeaa0f8314de5da204db6a7dcab87, 0x0074e9eb63f38cd5176e7f146733f0373d80ac9955c1b5cb0d66d90e468f4602, 0x00917557eaccad6dc7a58932d57c6295289ba1776a70fd7f74b18f613b61ac93, 0x0110810aebf1179ffd59a2bca5cd984f4de4f67cd7af30cb3c511191579e8c61, 0x012946a295071f1aa82d09ab80dc38307596f4dc0b2e68d198be23fb407a5f15, 0x014a245714f06e9c8ceef0f0e4fecae3a6f1ad4d6b5da2da9f9aaf0ad32ed5da, 0x0154bb7d10b5edbc5be81513e339d16fe8de09f998eab0513ae51191c11abfec, 0x016e5cb2cad01fe303be3ca5c82ce2a220797d3bb29b5e036aac0e799c1e6c5f]
2025-03-25T20:37:57.399599Z DEBUG sui_gas_station::sui_client: Got updated gas coin info: GasCoin { object_ref: (0x00522ef2ca8b295b2025368fd7a6a08bb4ef69c25d9a7562a5efb4ef6c0d2241, SequenceNumber(3), o#HBLxUCnbAs1pzV78QwePYheSyFo8xiUd9gS3sWx6bV9V), balance: 100004829 }
...
2025-03-25T20:37:57.399646Z DEBUG sui_gas_station::gas_pool::gas_pool_core: Trying to release gas coins: [GasCoin { object_ref: (0x0154bb7d10b5edbc5be81513e339d16fe8de09f998eab0513ae51191c11abfec, SequenceNumber(3), o#23eCmW5KjGWdZb2BTZ3XBwX7Hv8MGPfQ74qFAfHTpPV2), balance: 100004829 }, GasCoin { object_ref: (0x016e5cb2cad01fe303be3ca5c82ce2a220797d3bb29b5e036aac0e799c1e6c5f, SequenceNumber(3), o#ECAfi8RVDsmCi3vvTBhEWEYidsVarvafJTUdeY2sMYxz), balance: 100004829 }, GasCoin { object_ref: (0x0074e9eb63f38cd5176e7f146733f0373d80ac9955c1b5cb0d66d90e468f4602, SequenceNumber(3), o#HdgyWEagnprLBFeNxYDjixH3ehrzh1YzkuffTenrcLrX), balance: 100004829 }, GasCoin { object_ref: (0x0110810aebf1179ffd59a2bca5cd984f4de4f67cd7af30cb3c511191579e8c61, SequenceNumber(3), o#6qic7aP6K4b25WRdPoaCHEDxYEQWfgTg85youkWUeQxu), balance: 100004829 }, GasCoin { object_ref: (0x014a245714f06e9c8ceef0f0e4fecae3a6f1ad4d6b5da2da9f9aaf0ad32ed5da, SequenceNumber(3), o#CJJsMYcaVbCmQoFYjtbe94RrFVXmS7kb9ZYsrrMhUkMN), balance: 100004829 }, GasCoin { object_ref: (0x0061c6a045715742dcc0b9244bf4e8e0b2beeaa0f8314de5da204db6a7dcab87, SequenceNumber(3), o#HbdUSAyYn5ZgWfnJ3LgJeu37tF5anoztM21tMJVtr2wG), balance: 100004829 }, GasCoin { object_ref: (0x005ea99749895f7b7e4182f219cf027955e3c143eb02ebe133649a8706d7c124, SequenceNumber(3), o#AinhweRTLEvmGraCQhTYBh2WnpRA8iVxibn39qt14fgS), balance: 100004829 }, GasCoin { object_ref: (0x00522ef2ca8b295b2025368fd7a6a08bb4ef69c25d9a7562a5efb4ef6c0d2241, SequenceNumber(3), o#HBLxUCnbAs1pzV78QwePYheSyFo8xiUd9gS3sWx6bV9V), balance: 100004829 }, GasCoin { object_ref: (0x012946a295071f1aa82d09ab80dc38307596f4dc0b2e68d198be23fb407a5f15, SequenceNumber(3), o#2pUdsN2LMzo9sXVRe9BTCiwMkbBfdVSxpcmGcGGwV8fP), balance: 100004829 }, GasCoin { object_ref: (0x00917557eaccad6dc7a58932d57c6295289ba1776a70fd7f74b18f613b61ac93, SequenceNumber(3), o#ARUpkCFChnvedpGi54g8qcP2q3XD81HoyPseJZqAPZSw), balance: 100004829 }]
2025-03-25T20:37:57.400097Z DEBUG sui_gas_station::storage::redis: After add_new_coins. New total balance: 989983581200, new coin count: 9895
2025-03-25T20:37:57.400102Z  INFO sui_gas_station::gas_pool::gas_pool_core: Released 10 coins after expiration
```

</details>

## Using the Gas Coins

The `"gas_coins"` in the reservation response are to be used in the gas payment metadata of a Sui [Transaction], specifically, in [`GasPayment::objects`](https://docs.rs/sui-sdk-types/latest/sui_sdk_types/struct.GasPayment.html#structfield.objects).

## Signing Transactions

Once the transaction is built, to sign it one needs to:

1. [BCS]-serialize the Transaction into bytes
1. prepend an [Intent] to it (always `[0, 0, 0]` for user transactions on Sui)
1. hash the resulting bytes with Blake2b-256

The 3 steps above are implemented in Rust [here](https://docs.rs/sui-sdk-types/latest/src/sui_sdk_types/hash.rs.html#251-259). The result is what is known as the transaction [signing digest], which is simply a 32-byte array. See [`sui-crypto`] for all the supported signing schemes that can be applied to the signing digest. Read more about the resulting [user signature] format that Sui expects.

Since the `"gas_coins"` belong to an address, the `sponsor`, the [signing digest] needs to be signed by the sponsor. If `sender != sponsor`, that means 2 signatures will ultimately be send to the Sui fullnode, otherwise, just 1. See the documentation for [sui_executetransactionblock] for the full details of what the fullnode needs to execute a transaction.

## Submitting Transactions

If you are comfortable dealing with raw Sui transaction responses, then you can both sign the transaction for the sponsor and submit it to the network by using the `"/v1/execute_tx"` endpoint of the gas pool. See the [Main README] for details about that.

On the other hand, if you want to submit transaction through services like [iperps-api/ccxt/submit], which return you a parsed response with everything that happened onchain, then you'll have to sign for the `sponsor` outside of the gas pool and submit the transaction and signatures to the service.

[Main README]: ../README.md
[Transaction]: https://docs.rs/sui-sdk-types/latest/sui_sdk_types/struct.Transaction.html
[sui_executeTransactionBlock]: https://docs.sui.io/sui-api-ref#sui_executetransactionblock
[BCS]: https://docs.rs/sui-sdk-types/latest/sui_sdk_types/index.html#bcs
[Intent]: https://docs.rs/sui-sdk-types/latest/sui_sdk_types/struct.Intent.html
[signing digest]: https://docs.rs/sui-sdk-types/latest/sui_sdk_types/type.SigningDigest.html
[user signature]: https://docs.rs/sui-sdk-types/latest/sui_sdk_types/enum.UserSignature.html
[`sui-crypto`]: https://docs.rs/sui-crypto/latest/sui_crypto/
[iperps-api/ccxt/submit]: https://testnet.aftermath.finance/iperps-api/swagger-ui/#/Submit
