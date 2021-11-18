#!/bin/bash
set -eu

cargo build --lib -p zinzen_scheduler --target wasm32-unknown-unknown

wasm-bindgen target/wasm32-unknown-unknown/debug/zinzen_scheduler.wasm \
--out-dir ../website/ZinZen --no-modules --no-typescript