#!/usr/bin/env bash

set -e

# The value after 'Bearer' must be the same as used in the `GAS_STATION_AUTH` env var passed to the gas pool service
curl \
  -H 'Authorization: Bearer 123' \
  --json "{
  \"gas_budget\": $1,
  \"reserve_duration_secs\": 20
}" http://localhost:9527/v1/reserve_gas && echo
