#!/usr/bin/env bash
# Build and run the generated AWS CDK validation container through MGC.
# Build và chạy container kiểm tra AWS CDK được sinh qua MGC.
set -euo pipefail
project_root="${RUNNER_TEMP}/mgc-generated-clo-container"
mkdir -p "${project_root}"
chmod 0777 "${project_root}"
docker run --rm \
  --volume "${project_root}:/workspace" \
  --workdir /workspace \
  magicore-cli-smoke \
  create-clo cdk-typescript container-clo --docker
project="${project_root}/container-clo"
docker run --rm \
  --volume "${project}:/workspace" \
  --workdir /workspace \
  magicore-cli-smoke \
  install
test -s "${project}/mgc.lock"
sudo chown -R "$(id -u):$(id -g)" "${project}"
test "$(cat "${project}/.mgc.core")" = "clo"
grep -F 'COPY --from=mgc-cli /usr/local/bin/mgc' "${project}/Dockerfile"
grep -F 'RUN mgc install --frozen' "${project}/Dockerfile"
grep -F 'RUN mgc test' "${project}/Dockerfile"
grep -F 'RUN mgc build' "${project}/Dockerfile"
docker compose --file "${project}/docker-compose.yml" config --quiet
docker build \
  --build-arg MGC_IMAGE=magicore-cli-smoke \
  --file "${project}/Dockerfile" \
  --tag magicore-clo-cdk-container:local \
  "${project}"
docker run --rm magicore-clo-cdk-container:local
MGC_IMAGE_NAME=magicore-clo-cdk-container:local \
  MGC_DATA_VOLUME=magicore-clo-cdk-e2e-data \
  docker compose --file "${project}/docker-compose.yml" run --rm app
