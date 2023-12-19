#!/usr/bin/env bash

cargo check \
    && cargo +nightly fmt --all --check \
    && cargo test -- --nocapture \
    && cargo build
