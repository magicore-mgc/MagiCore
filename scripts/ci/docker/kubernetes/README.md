# Kubernetes deployment probe

`deploy_and_probe_web.sh` loads the already-built Web image into kind, applies the generated Deployment, waits for rollout, starts a local port-forward, verifies the served React page, and prints pod diagnostics on failure.

`deploy_and_probe_web.sh` nạp image Web đã build vào kind, apply Deployment được sinh, chờ rollout, mở port-forward nội bộ, xác minh trang React và in chẩn đoán pod khi lỗi.

The kind action and its cluster setup stay in the workflow before this script. Keep the existing service, deployment, port, selector, response marker, bounded wait and port-forward cleanup trap.
