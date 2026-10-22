#!/usr/bin/env bash
# Build and run the generated Rust library container with MGC-owned packages.
# Build và chạy container thư viện Rust với package do MGC quản lý.
set -euo pipefail
project_root="${RUNNER_TEMP}/mgc-generated-lib-container"
mkdir -p "${project_root}"
chmod 0777 "${project_root}"
docker run --rm \
  --volume "${project_root}:/workspace" \
  --workdir /workspace \
  magicore-cli-smoke \
  create-lib rust container-lib --docker
project="${project_root}/container-lib"
docker run --rm \
  --volume "${project}:/workspace" \
  --workdir /workspace \
  magicore-cli-smoke \
  install
test -s "${project}/mgc.lock"
sudo chown -R "$(id -u):$(id -g)" "${project}"
test "$(cat "${project}/.mgc.core")" = "lib"
grep -F 'COPY --from=mgc-cli /usr/local/bin/mgc' "${project}/Dockerfile"
grep -F 'RUN mgc install --frozen' "${project}/Dockerfile"
grep -F 'RUN mgc test' "${project}/Dockerfile"
grep -F 'RUN mgc build' "${project}/Dockerfile"
docker compose --file "${project}/docker-compose.yml" config --quiet
docker build \
  --build-arg MGC_IMAGE=magicore-cli-smoke \
  --file "${project}/Dockerfile" \
  --tag magicore-lib-rust-container:local \
  "${project}"
docker run --rm magicore-lib-rust-container:local
MGC_IMAGE_NAME=magicore-lib-rust-container:local \
  MGC_DATA_VOLUME=magicore-lib-rust-e2e-data \
  docker compose --file "${project}/docker-compose.yml" run --rm app
