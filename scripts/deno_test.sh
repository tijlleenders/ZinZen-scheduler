#!/usr/bin/env bash

set -euo pipefail

# Build
echo -n "Build..."
wasm-pack build --release --target deno

# Run JS test
echo "Running deno tests"
deno test --allow-read tests/*
