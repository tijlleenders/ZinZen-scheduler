#!/bin/bash

set -euo pipefail

TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/scheduler.wasm

# Build
cargo build --target $TARGET --release

# Process final binary
wasm-strip $BINARY
mkdir -p test
wasm-opt -o ts/scheduler.wasm -O3 $BINARY
ls -lh ts/scheduler.wasm

# Finally execute wasm inside Deno
deno run --allow-read test/ci_test.ts
