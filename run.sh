#!/bin/bash

set -e

here=$(pwd)
root_dir=$(dirname "$here")

export DUCKDB_LIB_DIR="$root_dir/duckdb-lib"
export DUCKDB_INCLUDE_DIR=$DUCKDB_LIB_DIR
export LD_LIBRARY_PATH=$DUCKDB_LIB_DIR

export DUCKDB_STATIC=1
export RUSTFLAGS="-C link-arg=-lstdc++"

rm -rf ./target

cargo build --release
./target/release/quack-in-rust
