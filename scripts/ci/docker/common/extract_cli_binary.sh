#!/usr/bin/env bash
# Extract the built CLI binary from the image for host-side SDK-aware operations.
# Trích binary CLI đã build từ image để chạy thao tác SDK-aware trên host.
set -euo pipefail
docker create --name magicore-cli-extract magicore-cli-smoke
docker cp magicore-cli-extract:/usr/local/bin/mgc "$RUNNER_TEMP/mgc"
docker rm magicore-cli-extract
chmod 0755 "$RUNNER_TEMP/mgc"
"$RUNNER_TEMP/mgc" --version
