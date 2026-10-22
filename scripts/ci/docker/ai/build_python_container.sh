#!/usr/bin/env bash
# Build and run the generated AI Python image with MGC-owned packages.
# Build và chạy image AI Python được sinh với package do MGC quản lý.
set -euo pipefail
project_root="${RUNNER_TEMP}/mgc-generated-ai-container"
mkdir -p "${project_root}"
chmod 0777 "${project_root}"
docker run --rm \
  --volume "${project_root}:/workspace" \
  --workdir /workspace \
  magicore-cli-smoke \
  create-ai python-agent container-ai --docker
project="${project_root}/container-ai"
docker run --rm \
  --volume "${project}:/workspace" \
  --workdir /workspace \
  magicore-cli-smoke \
  install-ai
test -s "${project}/mgc.lock"
sudo chown -R "$(id -u):$(id -g)" "${project}"
test "$(cat "${project}/.mgc.core")" = "ai"
grep -F 'COPY --from=mgc-cli /usr/local/bin/mgc' "${project}/Dockerfile"
grep -F 'pytest==9.1.1' "${project}/pyproject.toml"
grep -F 'RUN mgc install --frozen' "${project}/Dockerfile"
grep -F 'RUN mgc test' "${project}/Dockerfile"
docker compose --file "${project}/docker-compose.yml" config --quiet
docker build \
  --build-arg MGC_IMAGE=magicore-cli-smoke \
  --file "${project}/Dockerfile" \
  --tag magicore-ai-python-container:local \
  "${project}"
docker run --rm magicore-ai-python-container:local
MGC_IMAGE_NAME=magicore-ai-python-container:local \
  MGC_DATA_VOLUME=magicore-ai-python-e2e-data \
  docker compose --file "${project}/docker-compose.yml" run --rm app
