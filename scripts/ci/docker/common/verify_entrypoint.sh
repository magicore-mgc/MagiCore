#!/usr/bin/env bash
# Verify the CLI image runs as its configured non-root user.
# Xác minh image CLI chạy bằng user không-root đã cấu hình.
set -e -o pipefail
test "$(docker image inspect --format '{{.Config.User}}' magicore-cli-smoke)" = "65532:65532"
docker run --rm magicore-cli-smoke --version
docker run --rm magicore-cli-smoke --help
