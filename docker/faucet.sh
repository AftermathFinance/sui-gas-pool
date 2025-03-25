#!/usr/bin/env bash

set -e

function faucet {
  curl --json "{
    \"FixedAmountRequest\": {
      \"recipient\": \"$1\"
    }
  }" http://localhost:9123/v1/gas && echo
}

faucet $1
