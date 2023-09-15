#!/bin/bash

set -euo pipefail

TARGET=wasm32-unknown-unknown
CARGO_BINARY=target/$TARGET/release/scheduler.wasm
BINARY=js-api/scheduler.wasm

# Build
echo -n "Build..."
# remove the -q flag to see cargo ouput
cargo build --target --features generate-tests $TARGET --release

# wasm-bindgen
echo -n "Bindgen..."
mkdir -p js-api
cp $CARGO_BINARY $BINARY
wasm-bindgen $BINARY --out-dir js-api/ --target deno --reference-types --no-typescript
echo " ok"

## Process final binary
#echo -n "Optimizing wasm..."
#wasm-strip $BINARY
#wasm-opt -o js-api/scheduler.wasm -O3 $BINARY
#echo " ok"

# Run JS test
echo "Running deno tests"
deno test --allow-read tests/*
