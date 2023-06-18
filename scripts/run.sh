#!/bin/bash

cd "$(dirname "$0")/.."

# Prevents recompiling dependencies on every run
export CARGO_TARGET_DIR=../target/

RUSTFLAGS="$RUSTFLAGS -A dead_code" cargo run

exit 0
