#!/bin/bash

# Step in to parent directory
cd "$(dirname "$0")/.."

# Run program
RUSTFLAGS="$RUSTFLAGS -A dead_code" cargo run

exit 0
