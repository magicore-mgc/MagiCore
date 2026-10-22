#!/usr/bin/env bash
# Verify that install output names its shared-store cache source.
# Xác minh output install nêu đúng nguồn cache shared store.
set -euo pipefail
if [[ "$RUNNER_OS" == "Windows" ]]; then MGCBIN="$HOME/.magicore-test/bin/mgc.exe"; else MGCBIN="$HOME/.magicore-test/bin/mgc"; fi
WORK="$(mktemp -d)"
cd "$WORK"
"$MGCBIN" create-lib rust store-label
cd store-label
"$MGCBIN" install > install_label.log 2>&1
cat install_label.log
grep -q "cache source: shared mgc store" install_label.log
