#!/usr/bin/env bash
# Create the official Web initializer and verify provenance, install, test, build and strict audit.
# Tạo initializer Web chính thức rồi kiểm tra provenance, install, test, build và audit strict.
set -euo pipefail
cd "$WORK"
CREATE_OUTPUT="$("$MGCBIN" create-web react test-web 2>&1)" || {
  printf '%s\n' "$CREATE_OUTPUT"
  exit 1
}
printf '%s\n' "$CREATE_OUTPUT"
cd test-web
# The product does not print an "installing initializer" banner any more: the
# initializer runs as a registry package and its identity is recorded in the
# provenance file. The gate must assert the durable evidence (provenance), not a
# log line that the product no longer emits.
# (Sản phẩm không in banner "installing initializer" nữa: initializer chạy như
# package registry và danh tính được ghi vào file provenance. Gate phải kiểm
# bằng chứng bền (provenance), không phải dòng log sản phẩm không còn in.)
if ! grep -Eq 'npm:create-vite@[0-9]+\.[0-9]+\.[0-9]+#generator=vite:react-ts' .mgc-provenance.json; then
  echo "release E2E scaffold provenance does not identify its online generator" >&2
  exit 1
fi
"$MGCBIN" install
# A missing test command must fail this release gate; recording
# absence and continuing would be a false-green.
# Thiếu test command phải chặn gate phát hành, không được ghi chú rồi đi tiếp.
if ! grep -q '"test"[[:space:]]*:' package.json; then
  echo "release E2E scaffold has no test script" >&2
  exit 1
fi
"$MGCBIN" test
"$MGCBIN" build
test -d dist || test -d build
# STRICT audit: exit 0 (clean) or 1 (findings) are audit
# results; exit 2 means UNVERIFIED — the scanner/tool did not
# run, and this lane must never pass on that (P0 finding #6).
# Audit STRICT: exit 0 (sạch) hoặc 1 (có finding) là kết quả
# audit; exit 2 nghĩa là CHƯA XÁC THỰC — scanner/tool chưa
# chạy, lane không bao giờ được xanh trên trạng thái đó.
export MGC_AUDIT_STRICT=1
set +e
"$MGCBIN" audit
rc=$?
set -e
echo "MARKER audit rc=$rc"
if [[ "$rc" -ne 0 && "$rc" -ne 1 ]]; then
  echo "web audit exited $rc (0/1 allowed, 2=unverified is a FAIL)" >&2
  exit 1
fi
unset MGC_AUDIT_STRICT
