#!/usr/bin/env bash

set -euo pipefail

echo 'Checking that the build worked.'
echo -n '1. The final wasm file is not empty:'

EXPECTED_FINAL_WASM_FILE_PATH="js-api/src/scheduler_bg.wasm";
WASM_FILE_SIZE=`wc -c $EXPECTED_FINAL_WASM_FILE_PATH | sed 's/ .*$//'`

if [[ $WASM_FILE_SIZE == '0' ]]; then
    echo "Check phase failed: expected a non empty EXPECTED_FINAL_WASM_FILE_PAT"
    exit 1
fi

echo ' ok.'