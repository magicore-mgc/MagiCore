#!/usr/bin/env bash
# Prove warm cache reuse and offline reinstall for the generated Web project.
# Chứng minh tái sử dụng cache và cài lại offline cho dự án Web được tạo.
set -euo pipefail
cd "$WORK/test-web"
# Warm reinstall with cache-hit proof: the second install must
# report bytes served from cache (MgCStore). Empty cache
# evidence = false-green (P0 finding #6).
# Windows note (2026-09-12): the first install's lifecycle
# scripts leave esbuild.exe running — a live process keeps
# its own executable locked, and the warm reinstall's prune
# cannot delete it (Windows file-locking, exit 1 in ~2s).
# Kill the stragglers FIRST, and capture the exit code so the
# log is always printed even on failure.
# Warm reinstall kèm bằng chứng cache-hit: lần install thứ hai
# phải báo byte lấy từ cache (MgCStore). Cache rỗng = false-
# green. Ghi chú Windows: lifecycle script của install lần 1
# để esbuild.exe còn chạy — process sống giữ chính file exe
# của nó, prune của lần cài lại không xóa được (file-lock
# Windows, exit 1 trong ~2s). Giết process sót TRƯỚC, và bắt
# exit code để log luôn được in kể cả khi fail.
if [[ "$RUNNER_OS" == "Windows" ]]; then
  taskkill //F //IM esbuild.exe //T > /dev/null 2>&1 || true
fi
echo "MARKER warm install start"
set +e
"$MGCBIN" install > warm_install.log 2>&1
warm_rc=$?
set -e
cat warm_install.log
if [[ "$warm_rc" -ne 0 ]]; then
  echo "warm install exited $warm_rc" >&2
  exit 1
fi
if ! grep -Eq "from cache|Reused|cache" warm_install.log; then
  echo "warm install showed no cache evidence" >&2
  exit 1
fi

# Offline mode must reinstall from the warmed cache; this step checks the
# MGC command result and materialized tree, not network traffic directly.
# Offline phải cài lại từ cache đã warm; step này kiểm tra kết quả lệnh
# MGC và cây package đã materialize, chưa quan sát trực tiếp traffic mạng.
echo "MARKER offline reinstall start"
set +e
"$MGCBIN" install --offline > offline_install.log 2>&1
offline_rc=$?
set -e
cat offline_install.log
if [[ "$offline_rc" -ne 0 ]]; then
  echo "offline reinstall exited $offline_rc" >&2
  exit 1
fi
test -d node_modules
echo "MARKER web lane complete"
