#!/usr/bin/env bash
set -eu

cargo check --all
cargo fmt --all -- --check
cargo clippy --all -- -D warnings