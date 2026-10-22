#!/usr/bin/env bash
# Run the scaffold start or preview task and verify its HTTP response and shutdown.
# Chạy task start hoặc preview của scaffold rồi xác minh HTTP và dừng process.
set -euo pipefail
cd "$WORK/test-web"
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/probe_process.sh"
# run: the scaffold's own start/preview script through the
# native task runner (P0 finding #6 — run was never executed).
# The server is long-lived: spawn it in the background, probe
# for a REAL HTTP roundtrip, then kill the tree — a foreground
# run would hit the 300s exec timeout and die as 143 (bug
# caught on the first E2E run, 2026-09-12).
# run: script start/preview của template qua task runner native
# (P0 finding #6 — run chưa bao giờ được chạy). Server dài hạn:
# spawn nền, chờ HTTP roundtrip THẬT, rồi giết cây — chạy
# foreground sẽ dính timeout 300s của exec và chết 143 (bug
# bắt được ở lần E2E đầu tiên).
if grep -q '"start"[[:space:]]*:' package.json; then
  RUN_SCRIPT=start
elif grep -q '"preview"[[:space:]]*:' package.json; then
  RUN_SCRIPT=preview
else
  echo "release E2E scaffold has neither start nor preview run script" >&2
  exit 1
fi
echo "MARKER run probe start script=$RUN_SCRIPT"
"$MGCBIN" run "$RUN_SCRIPT" > run.log 2>&1 &
RUN_PID=$!
run_ok=false
for i in $(seq 1 60); do
  if curl -sf -o /dev/null "http://127.0.0.1:4315/" || curl -sf -o /dev/null "http://localhost:4315/"; then run_ok=true; break; fi
  if ! kill -0 $RUN_PID 2>/dev/null; then break; fi
  sleep 2
done
echo "MARKER run probe done ok=$run_ok"
if [[ "$run_ok" != true ]]; then
  echo "run start did not answer HTTP on port 4315" >&2
  cat run.log >&2 || true
  stop_probe_process "$RUN_PID"
  exit 1
fi
stop_probe_process "$RUN_PID"
run_port_closed=false
for i in $(seq 1 15); do
  if ! port_responds; then
    run_port_closed=true
    break
  fi
  sleep 1
done
if [[ "$run_port_closed" != true ]]; then
  echo "port 4315 still serving after run shutdown — leak" >&2
  cat run.log >&2 || true
  exit 1
fi
