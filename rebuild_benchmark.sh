#!/bin/bash
set -x
set -e

rm -rf src/bin
mkdir src/bin
cp tests/benchmark.rs src/bin/benchmark.rs

sed -i -E 's/\[dev-dependencies\]//g' Cargo.toml

cargo build --release

./target/release/benchmark 100000
