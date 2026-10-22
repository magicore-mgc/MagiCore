#!/usr/bin/env bash
# Deploy the generated Web container to kind and verify its HTTP service.
# Deploy container Web lên kind và xác minh HTTP service.
set -euo pipefail
project="${RUNNER_TEMP}/mgc-generated-web-container/container-web"
kind load docker-image magicore-container-web:local
kubectl apply --filename "${project}/k8s/deployment.yaml"
kubectl rollout status deployment/magicore-container-web --timeout=180s
kubectl port-forward --address 127.0.0.1 service/magicore-container-web 4315:4315 \
  >"${RUNNER_TEMP}/mgc-k8s-port-forward.log" 2>&1 &
port_forward_pid=$!
trap 'kill "${port_forward_pid}" 2>/dev/null || true' EXIT
for attempt in $(seq 1 60); do
  if curl --fail --silent http://127.0.0.1:4315/ | grep -F '<title>container-web</title>'; then
    exit 0
  fi
  sleep 2
done
cat "${RUNNER_TEMP}/mgc-k8s-port-forward.log"
kubectl describe pod --selector=app.kubernetes.io/name=magicore-container-web
exit 1
