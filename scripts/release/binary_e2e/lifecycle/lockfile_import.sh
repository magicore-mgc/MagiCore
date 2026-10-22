#!/usr/bin/env bash
# Verify importing an existing Bun lockfile into the MGC lock graph.
# Xác minh nhập lockfile Bun hiện có thành đồ thị lock của MGC.
set -euo pipefail
if [[ "$RUNNER_OS" == "Windows" ]]; then MGCBIN="$HOME/.magicore-test/bin/mgc.exe"; else MGCBIN="$HOME/.magicore-test/bin/mgc"; fi
WORK="$(mktemp -d)"
mkdir -p "$WORK/proj"
cd "$WORK/proj"
cat > bun.lock <<'LOCK'
{
  "packages": {
    "lodash": ["lodash@4.17.20", "", {}, "sha512-LD"],
  },
}
LOCK
"$MGCBIN" import
test -f mgc.lock
grep -q "lodash" mgc.lock
