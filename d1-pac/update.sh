#!/usr/bin/env bash
set -x
set -e

cd ..
cargo xtask codegen d1
