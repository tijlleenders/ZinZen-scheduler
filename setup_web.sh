#!/bin/bash

## On linux requires "sudo apt-get install pkg-config libssl-dev" ##not sure if pkg-config also required

set -eu
rustup target add wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli
cargo update -p wasm-bindgen