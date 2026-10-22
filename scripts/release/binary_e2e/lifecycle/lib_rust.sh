#!/usr/bin/env bash
# Exercise the installed binary through the Rust library lifecycle.
# Chạy binary đã cài qua vòng đời thư viện Rust.
set -euo pipefail
if [[ "$RUNNER_OS" == "Windows" ]]; then MGCBIN="$HOME/.magicore-test/bin/mgc.exe"; else MGCBIN="$HOME/.magicore-test/bin/mgc"; fi
WORK="$(mktemp -d)"
cd "$WORK"
"$MGCBIN" create-lib rust test-lib
cd test-lib
"$MGCBIN" install
"$MGCBIN" test
"$MGCBIN" build
export MGC_AUDIT_STRICT=1
set +e
"$MGCBIN" audit
rc=$?
set -e
if [[ "$rc" -ne 0 && "$rc" -ne 1 ]]; then
  echo "lib audit exited $rc (0/1 allowed, 2=unverified is a FAIL)" >&2
  exit 1
fi
unset MGC_AUDIT_STRICT
