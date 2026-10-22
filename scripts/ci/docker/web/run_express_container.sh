#!/usr/bin/env bash
# Verify the generated Express service through a published Docker port.
# Xác minh Express được sinh qua port mapping của Docker.
set -euo pipefail
docker run --detach --name magicore-generated-express-e2e \
  --publish 127.0.0.1:4351:4315 \
  magicore-express-container:local
trap 'docker rm --force magicore-generated-express-e2e >/dev/null 2>&1 || true' EXIT
for attempt in $(seq 1 60); do
  if curl --fail --silent http://127.0.0.1:4351/ | grep -F 'Hello from MagiCore Express!'; then
    exit 0
  fi
  sleep 2
done
docker logs magicore-generated-express-e2e
exit 1
