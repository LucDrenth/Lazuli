#!/bin/bash

cd "$(dirname "$0")/.."

RUSTFLAGS="$RUSTFLAGS -A dead_code" cargo run

exit 0
