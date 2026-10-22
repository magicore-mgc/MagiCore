#!/usr/bin/env bash
# Build and run the Flutter container with MGC-owned Pub resolution.
# Build và chạy container Flutter với Pub dependency do MGC quản lý.
set -euo pipefail
project_root="${RUNNER_TEMP}/mgc-generated-app-container"
mkdir -p "${project_root}"
chmod 0777 "${project_root}"
docker run --rm \
  --volume "${project_root}:/workspace" \
  --workdir /workspace \
  magicore-cli-smoke \
  create-app flutter container-app --docker
project="${project_root}/container-app"
sudo chown -R "$(id -u):$(id -g)" "${project}"
mkdir -p "${RUNNER_TEMP}/mgc-app-home" "${RUNNER_TEMP}/mgc-app-data" "${RUNNER_TEMP}/mgc-app-cache" "${RUNNER_TEMP}/mgc-app-templates"
(
  cd "${project}"
  HOME="${RUNNER_TEMP}/mgc-app-home" \
  MGC_DATA_DIR="${RUNNER_TEMP}/mgc-app-data" \
  MGC_CACHE_DIR="${RUNNER_TEMP}/mgc-app-cache" \
  MGC_TEMPLATES_DIR="${RUNNER_TEMP}/mgc-app-templates" \
    "${RUNNER_TEMP}/mgc" install
)
test -s "${project}/mgc.lock"
test "$(cat "${project}/.mgc.core")" = "app"
grep -F 'COPY --from=mgc-cli /usr/local/bin/mgc' "${project}/Dockerfile"
grep -F 'FLUTTER_VERSION=3.47.7' "${project}/Dockerfile"
grep -F 'sha256sum --check --strict' "${project}/Dockerfile"
grep -F 'RUN mgc install --frozen && mgc test && mgc build' "${project}/Dockerfile"
if grep -F 'dart pub get' "${project}/Dockerfile"; then
  echo 'Flutter dependency installation bypasses MGC' >&2
  exit 1
fi
docker compose --file "${project}/docker-compose.yml" config --quiet
docker build \
  --build-arg MGC_IMAGE=magicore-cli-smoke \
  --file "${project}/Dockerfile" \
  --tag magicore-app-flutter-container:local \
  "${project}"
docker run --rm magicore-app-flutter-container:local
MGC_IMAGE_NAME=magicore-app-flutter-container:local \
  MGC_DATA_VOLUME=magicore-app-flutter-e2e-data \
  docker compose --file "${project}/docker-compose.yml" run --rm app
