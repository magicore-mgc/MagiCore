#!/usr/bin/env bash
# Probe a real Web dev HTTP response and verify complete shutdown.
# Probe HTTP thật của Web dev và xác minh tiến trình đã dừng hoàn toàn.
set -euo pipefail
cd "$WORK/test-web"
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/probe_process.sh"
# dev probe: start mgc dev on a 4315-permutation port, wait
# for a REAL HTTP roundtrip, then shut it down cleanly —
# --help proves nothing (P0 finding #6).
# dev probe: khởi động mgc dev trên port hoán vị 4315, chờ
# HTTP roundtrip THẬT, rồi tắt sạch — --help không chứng minh
# được gì.
if port_responds; then
  echo "port 4315 was already serving before mgc dev; refusing contaminated lifecycle probe" >&2
  exit 1
fi
if [[ "$RUNNER_OS" == "Windows" ]]; then
  "$MGCBIN" dev --port 4315 > dev.log 2>&1 &
  DEV_PID=$!
else
  "$MGCBIN" dev --port 4315 > dev.log 2>&1 &
  DEV_PID=$!
fi
echo "MARKER dev spawned pid=$DEV_PID"
probe_ok=false
for i in $(seq 1 60); do
  # Probe both stacks: vite binds ::1 (IPv6 localhost) on some
  # hosts and 127.0.0.1 on others.
  # Probe cả hai stack: vite bind ::1 (IPv6 localhost) trên
  # một số máy và 127.0.0.1 trên máy khác.
  if curl -sf -o /dev/null "http://127.0.0.1:4315/" || curl -sf -o /dev/null "http://localhost:4315/"; then probe_ok=true; break; fi
  sleep 2
done
if [[ "$probe_ok" != true ]]; then
  echo "dev server did not answer HTTP on port 4315 within 120s" >&2
  cat dev.log >&2 || true
  stop_probe_process "$DEV_PID"
  exit 1
fi
# Clean shutdown: the dev tree (mgc → node → vite) must die
# WITHOUT leaving the port bound. Killing only the parent
# leaks the node child holding the port (REVIEW round 2
# finding, 2026-09-12) — kill the whole tree.
# Tắt sạch: cả cây dev (mgc → node → vite) phải chết, không để
# port còn bị bind. Chỉ giết cha sẽ lộ node con giữ port —
# giết cả cây.
if [[ "$RUNNER_OS" == "Windows" ]]; then
  stop_probe_process "$DEV_PID"
else
  kill "$DEV_PID" 2>/dev/null || true
  for i in $(seq 1 15); do
    if ! kill -0 $DEV_PID 2>/dev/null; then break; fi
    sleep 1
  done
  if kill -0 $DEV_PID 2>/dev/null; then kill -9 $DEV_PID 2>/dev/null || true; fi
  # Sweep leaked vite/node children of this project sandbox.
  # Quét dọn con vite/node sót lại của sandbox project này.
  pkill -f "vite.*4315" 2>/dev/null || true
  sleep 1
fi

# Verify shutdown on every OS before allowing `mgc run` to use
# the same port. Otherwise a leaked dev server can make the run
# probe pass without the run process ever serving a request.
# `mgc run` must not pass because the previous dev server is still running.
# `mgc run` không được pass nhờ dev server còn chạy từ pha trước.
# Verify that the port is released on every operating system.
# Kiểm tra cổng đã được nhả trên mọi hệ điều hành.
dev_port_closed=false
for i in $(seq 1 15); do
  if ! port_responds; then
    dev_port_closed=true
    break
  fi
  sleep 1
done
if [[ "$dev_port_closed" != true ]]; then
  echo "port 4315 still serving after dev shutdown — run probe would be a false positive" >&2
  cat dev.log >&2 || true
  stop_probe_process "$DEV_PID"
  exit 1
fi
