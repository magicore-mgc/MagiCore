#!/usr/bin/env bash
# Build and run Go, Python, and TypeScript library containers through MGC.
# Build và chạy container thư viện Go, Python và TypeScript qua MGC.
set -euo pipefail
for language in go python typescript; do
  project_name="container-lib-${language}"
  project_root="${RUNNER_TEMP}/mgc-generated-${project_name}"
  mkdir -p "${project_root}"
  chmod 0777 "${project_root}"
  docker run --rm \
    --volume "${project_root}:/workspace" \
    --workdir /workspace \
    magicore-cli-smoke \
    create-lib "${language}" "${project_name}" --docker
  project="${project_root}/${project_name}"
  image="magicore-lib-${language}-container:local"
  docker run --rm \
    --volume "${project}:/workspace" \
    --workdir /workspace \
    magicore-cli-smoke \
    install
  test -s "${project}/mgc.lock"
  sudo chown -R "$(id -u):$(id -g)" "${project}"
  test "$(cat "${project}/.mgc.core")" = "lib"
  docker compose --file "${project}/docker-compose.yml" config --quiet
  grep -F 'COPY --from=mgc-cli /usr/local/bin/mgc' "${project}/Dockerfile"
  grep -F 'RUN mgc test' "${project}/Dockerfile"
  grep -F 'RUN mgc build' "${project}/Dockerfile"
  docker build \
    --build-arg MGC_IMAGE=magicore-cli-smoke \
    --file "${project}/Dockerfile" \
    --tag "${image}" \
    "${project}"
  docker run --rm "${image}"
  MGC_IMAGE_NAME="${image}" \
    MGC_DATA_VOLUME="magicore-lib-${language}-e2e-data" \
    docker compose --file "${project}/docker-compose.yml" run --rm app
done
