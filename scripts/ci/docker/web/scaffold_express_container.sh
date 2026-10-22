#!/usr/bin/env bash
# Scaffold and build the MGC-owned Express container.
# Scaffold và build container Express do MGC quản lý.
set -euo pipefail
project_root="${RUNNER_TEMP}/mgc-generated-express-container"
mkdir -p "${project_root}"
chmod 0777 "${project_root}"
docker run --rm \
  --volume "${project_root}:/workspace" \
  --workdir /workspace \
  magicore-cli-smoke \
  create-web express container-express --docker
project="${project_root}/container-express"
docker run --rm \
  --volume "${project}:/workspace" \
  --workdir /workspace \
  magicore-cli-smoke \
  install-web
test -s "${project}/mgc.lock"
sudo chown -R "$(id -u):$(id -g)" "${project}"
test "$(cat "${project}/.mgc.core")" = "web"
grep -F "app.listen(port, '0.0.0.0'" "${project}/src/server.ts"
grep -F 'RUN mgc install-web --frozen' "${project}/Dockerfile"
grep -F 'RUN mgc test' "${project}/Dockerfile"
docker build \
  --build-arg MGC_IMAGE=magicore-cli-smoke \
  --file "${project}/Dockerfile" \
  --tag magicore-express-container:local \
  "${project}"
