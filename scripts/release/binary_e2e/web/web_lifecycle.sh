#!/usr/bin/env bash
# Coordinate the Web lifecycle phases in one isolated temporary project.
# Điều phối các pha Web trên cùng một dự án tạm được cô lập.
set -euo pipefail
if [[ "$RUNNER_OS" == "Windows" ]]; then MGCBIN="$HOME/.magicore-test/bin/mgc.exe"; else MGCBIN="$HOME/.magicore-test/bin/mgc"; fi
WORK="$(mktemp -d)"
export MGCBIN WORK
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
bash "$SCRIPT_DIR/create_install_test_build_audit.sh"
bash "$SCRIPT_DIR/dev_server_probe.sh"
bash "$SCRIPT_DIR/run_script_probe.sh"
bash "$SCRIPT_DIR/cache_reinstall.sh"
