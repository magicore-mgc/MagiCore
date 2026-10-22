#!/usr/bin/env bash
# Install and build the seeded Web Node container fixture through MGC.
# Cài và build fixture container Web Node qua MGC.
set -euo pipefail
chmod 0777 tests/docker/web-node
docker run --rm \
  --volume "${PWD}/tests/docker/web-node:/workspace" \
  --workdir /workspace \
  magicore-cli-smoke \
  install-web
test -s tests/docker/web-node/mgc.lock
docker build \
  --build-arg MGC_IMAGE=magicore-cli-smoke \
  --file tests/docker/web-node/Dockerfile \
  --tag magicore-web-node-docker-e2e \
  tests/docker/web-node
