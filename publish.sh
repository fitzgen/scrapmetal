#!/usr/bin/env bash

set -eu

cd "$(dirname $0)"

cd ./scrapmetal-derive/
cargo publish --dry-run

cd ..
cargo publish --dry-run

cd ./scrapmetal-derive/
cargo publish

cd ..
cargo publish
