#!/usr/bin/env bash
# Run the generated React container and verify its served page.
# Chạy container React được sinh và xác minh trang được phục vụ.
set -euo pipefail
docker volume create magicore-generated-web-e2e-data >/dev/null
docker run --detach --name magicore-generated-web-e2e \
  --publish 127.0.0.1:4315:4315 \
  --volume magicore-generated-web-e2e-data:/mgc-data \
  magicore-container-web:local
trap 'docker rm --force magicore-generated-web-e2e >/dev/null 2>&1 || true; docker volume rm magicore-generated-web-e2e-data >/dev/null 2>&1 || true' EXIT
for attempt in $(seq 1 60); do
  if curl --fail --silent http://127.0.0.1:4315/ | grep -F '<title>container-web</title>'; then
    exit 0
  fi
  sleep 2
done
docker logs magicore-generated-web-e2e
exit 1
