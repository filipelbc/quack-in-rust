# Statically Linked DuckDB in Rust

This is sample code showing how to build a Rust program statically linked to DuckDB, including some extensions.

## How to use

Setup `vcpkg` in `../vcpkg`
```sh
git clone git@github.com:microsoft/vcpkg.git ../vcpkg
cd ../vcpkg
sh ./scripts/bootstrap.sh -disableMetrics
```

Clone `duckdb` into `../duckdb-src`, checkout desired commit.
```sh
git clone git@github.com:duckdb/duckdb.git ../duckdb-src
cd ../duckdb-src
git checkout v1.4.3
```

Install other build dependencies. See [duckdb docs](https://duckdb.org/docs/stable/dev/building/linux).

Run `./build-duckdb-libs.sh`. This will build a DuckDB static library into `../duckdb-lib`, including the desired extensions.

Run `./run.sh` to build and run the code.
