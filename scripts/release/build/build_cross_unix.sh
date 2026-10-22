#!/usr/bin/env bash
# Cross-target release build; caller owns toolchain setup and matrix selection.
set -euo pipefail

target="${MGC_RELEASE_TARGET:?MGC_RELEASE_TARGET is required}"
cross build --release --target "$target" --bin mgc --no-default-features --features all --locked
mkdir -p dist/magicore
cp "target/$target/release/mgc" dist/magicore/mgc
cross build --release --target "$target" --bin mgc --no-default-features --features web --locked
mkdir -p dist/magicore-web
cp "target/$target/release/mgc" dist/magicore-web/mgc
