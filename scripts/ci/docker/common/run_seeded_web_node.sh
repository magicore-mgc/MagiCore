#!/usr/bin/env bash
# Run the seeded Web Node image and verify install/test on its live container.
# Chạy image Web Node fixture và kiểm tra install/test trong container.
set -euo pipefail
docker volume create magicore-web-node-e2e-data >/dev/null
docker run --detach --name magicore-web-node-docker-e2e \
  --publish 4315:4315 \
  --volume magicore-web-node-e2e-data:/mgc-data \
  magicore-web-node-docker-e2e
trap 'docker rm --force magicore-web-node-docker-e2e >/dev/null 2>&1 || true; docker volume rm magicore-web-node-e2e-data >/dev/null 2>&1 || true' EXIT
for attempt in $(seq 1 30); do
  if docker exec magicore-web-node-docker-e2e mgc install-web --frozen && \
    docker exec --env MGC_CONTAINER_SMOKE=1 magicore-web-node-docker-e2e mgc test; then
    exit 0
  fi
  sleep 2
done
docker logs magicore-web-node-docker-e2e
exit 1
