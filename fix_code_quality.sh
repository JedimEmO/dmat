#!/usr/bin/env bash

set -eu

cargo fmt --all
cargo clippy --all --fix --allow-staged --allow-dirty