#!/bin/bash

set -euo pipefail

TARGET=wasm32-unknown-unknown
CARGO_BINARY=target/$TARGET/release/scheduler.wasm
BINARY=pkg/scheduler.wasm

# Build
echo -n "Build..."
# remove the -q flag to see cargo ouput
cargo build --target $TARGET # --release #For now we need to use debug so keep the default non-optimized

# wasm-bindgen
echo -n "Bindgen..."
mkdir -p pkg
cp $CARGO_BINARY $BINARY
wasm-bindgen $BINARY --reference-types --typescript --debug --keep-debug --out-dir pkg --target web
echo " ok"

# Process final binary
echo -n "Not yet optimizing wasm..."
# wasm-strip $BINARY
# wasm-opt -o pkg/scheduler.wasm -O3 $BINARY
# echo " ok"

echo "Please copy the ./pkg files to the ZinZen repository in the ./pkg directory"