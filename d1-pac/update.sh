#!/usr/bin/env bash
set -x
set -e

rm -rf src
mkdir src
svd2rust --target riscv -i ../svd/d1_unofficial.svd --const_generic
form -i lib.rs -o src
rm lib.rs
cargo fmt
