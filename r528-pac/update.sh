#!/usr/bin/env bash
set -x
set -e

rm -rf src
mkdir src
svdtools patch ../svd/r528_unofficial.yaml
svd2rust --target none -i ../svd/d1_unofficial.svd.patched --const_generic
form -i lib.rs -o src
rm lib.rs ../svd/d1_unofficial.svd.patched
cargo fmt
