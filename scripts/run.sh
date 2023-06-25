#!/bin/bash

# Step in to parent directory
cd "$(dirname "$0")/.."

# Prevent recompiling dependencies on every run
export CARGO_TARGET_DIR=./target/

# Run program
RUSTFLAGS="$RUSTFLAGS -A dead_code" cargo run

exit 0
