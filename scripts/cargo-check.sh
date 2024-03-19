#!/usr/bin/env bash

cargo check \
    && cargo +nightly fmt --all --check \
    && cargo test --tests e2e::after_12 -- --nocapture \
    && cargo build
