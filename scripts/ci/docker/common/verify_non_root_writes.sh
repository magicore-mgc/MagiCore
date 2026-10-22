#!/usr/bin/env bash
# Check that each generated image runs as non-root and can write its app/data roots.
# Kiểm tra từng image sinh ra chạy non-root và ghi được vào thư mục app/data.
set -euo pipefail
for image in \
  magicore-container-web:local \
  magicore-express-container:local \
  magicore-ai-python-container:local \
  magicore-clo-cdk-container:local \
  magicore-lib-rust-container:local \
  magicore-lib-go-container:local \
  magicore-lib-python-container:local \
  magicore-lib-typescript-container:local \
  magicore-app-flutter-container:local; do
  docker run --rm --entrypoint /bin/sh "$image" -ec '
    test "$(id -u)" = 65532
    for root in /app /mgc-data; do
      probe="$(mktemp "$root/.mgc-write-probe.XXXXXX")"
      rm "$probe"
    done
  '
done
