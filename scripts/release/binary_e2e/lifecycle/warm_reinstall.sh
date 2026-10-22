#!/usr/bin/env bash
# Verify repeated install behavior using the installed binary.
# Xác minh cài lặp bằng binary đã cài.
set -euo pipefail
if [[ "$RUNNER_OS" == "Windows" ]]; then MGCBIN="$HOME/.magicore-test/bin/mgc.exe"; else MGCBIN="$HOME/.magicore-test/bin/mgc"; fi
WORK="$(mktemp -d)"
cd "$WORK"
"$MGCBIN" create-lib rust test-lib2
cd test-lib2
"$MGCBIN" install
# Warm reinstall: a second install over the same project must
# succeed (idempotency of the install pipeline through the
# installed binary).
"$MGCBIN" install
