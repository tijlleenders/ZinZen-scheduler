#!/usr/bin/env bash

cargo check \
    && cargo +nightly fmt --all --check \
    && cargo test \
    && cargo build
