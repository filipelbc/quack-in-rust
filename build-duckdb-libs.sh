#!/bin/bash

set -e

here=$(pwd)
root_dir=$(dirname "$here")

cd "$root_dir/duckdb-src"

export BUILD_EXTENSIONS='core_functions;httpfs;jemalloc;icu;json;parquet'

export EXTENSION_STATIC_BUILD=1
export ENABLE_EXTENSION_AUTOLOADING=1

export USE_MERGED_VCPKG_MANIFEST=1
export VCPKG_TOOLCHAIN_PATH="$root_dir/vcpkg/scripts/buildsystems/vcpkg.cmake"

export GEN=ninja

make bundle-library

mkdir -p "$root_dir/duckdb-lib"

cd "$root_dir/duckdb-lib"

rm -f -- *.a *.o *.h

cp "$root_dir/duckdb-src/src/include/duckdb.h" .

cp "$root_dir/duckdb-src/build/release/libduckdb_bundle.a" "libduckdb.a"

# Uncomment the code below for the workaround

#cp "$root_dir/duckdb-src/build/release/extension/CMakeFiles/duckdb_generated_extension_loader.dir/__/codegen/src/generated_extension_loader.cpp.o" .

#ar r libduckdb.a generated_extension_loader.cpp.o
