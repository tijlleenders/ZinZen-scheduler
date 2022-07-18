#!/bin/bash

set -euo pipefail

TARGET=wasm32-unknown-unknown
CARGO_BINARY=target/$TARGET/release/scheduler.wasm
BINARY=js-api/scheduler.wasm

# Build
cargo build --target $TARGET --release

# wasm-bindgen
mkdir -p js-api
cp $CARGO_BINARY $BINARY
wasm-bindgen $BINARY --out-dir js-api/ --target deno

# Process final binary
wasm-strip $BINARY
wasm-opt -o js-api/scheduler.wasm -O3 $BINARY
du -sh js-api/scheduler.wasm

# Run JS test
deno test --allow-read js-tests/*