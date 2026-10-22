#!/usr/bin/env bash
# Scaffold the React Web container through MGC and verify its generated contract.
# Scaffold container Web React qua MGC rồi xác minh hợp đồng sinh ra.
set -euo pipefail
project_root="${RUNNER_TEMP}/mgc-generated-web-container"
mkdir -p "${project_root}"
chmod 0777 "${project_root}"
docker run --rm \
  --volume "${project_root}:/workspace" \
  --workdir /workspace \
  magicore-cli-smoke \
  create-web react-vite container-web --k8s
project="${project_root}/container-web"
docker run --rm \
  --volume "${project}:/workspace" \
  --workdir /workspace \
  magicore-cli-smoke \
  install-web
test -s "${project}/mgc.lock"
# The CLI image writes project metadata as UID 65532:65532. The
# runner's Docker client must be able to read this build context.
# (CLI image ghi metadata dưới UID 65532:65532; Docker client runner cần đọc được build context.)
sudo chown -R "$(id -u):$(id -g)" "${project}"
test "$(cat "${project}/.mgc.core")" = "web"
test -r "${project}/mgc.toml"
grep -F 'RUN mgc install-web --frozen' "${project}/Dockerfile"
grep -F 'RUN mgc test' "${project}/Dockerfile"
grep -F 'ENTRYPOINT ["/usr/local/bin/mgc-container-entrypoint"]' "${project}/Dockerfile"
grep -F 'CMD ["dev", "--host", "0.0.0.0"' "${project}/Dockerfile"
grep -F 'mgc install-web --frozen' "${project}/mgc-container-entrypoint"
docker compose --file "${project}/docker-compose.yml" config --quiet
docker build \
  --build-arg MGC_IMAGE=magicore-cli-smoke \
  --file "${project}/Dockerfile" \
  --tag magicore-container-web:local \
  "${project}"
