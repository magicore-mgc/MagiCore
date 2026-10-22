#!/usr/bin/env bash
# Standard native-target release build.
# Keep GOARCH/CGO_ENABLED target-specific. Do not export CC/CFLAGS globally:
# esbuild-rs needs cgo for foreign Go targets, while global architecture flags
# break host proc-macros and ring. For Darwin, Go adds the target -arch itself.
set -euo pipefail

target="${MGC_RELEASE_TARGET:?MGC_RELEASE_TARGET is required}"
cargo build --release --target "$target" --bin mgc --no-default-features --features all --locked
mkdir -p dist/magicore
cp "target/$target/release/mgc" dist/magicore/mgc
cargo build --release --target "$target" --bin mgc --no-default-features --features web --locked
mkdir -p dist/magicore-web
cp "target/$target/release/mgc" dist/magicore-web/mgc
