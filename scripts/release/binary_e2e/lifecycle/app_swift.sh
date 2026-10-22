#!/usr/bin/env bash
# Exercise the installed binary through the Swift application lifecycle.
# Chạy binary đã cài qua vòng đời ứng dụng Swift.
set -euo pipefail
if [[ "$RUNNER_OS" == "Windows" ]]; then MGCBIN="$HOME/.magicore-test/bin/mgc.exe"; else MGCBIN="$HOME/.magicore-test/bin/mgc"; fi
WORK="$(mktemp -d)"
cd "$WORK"
swift --version
"$MGCBIN" create-app swift test-swift-app
cd test-swift-app
"$MGCBIN" install
# This fixture has no external dependencies, so install has no MGC lock graph to write.
# Fixture này không có dependency ngoài, nên install không cần ghi đồ thị vào mgc.lock.
"$MGCBIN" test -- --verbose 2>&1 | tee swift-test.log
grep -q "Executed 1 test" swift-test.log
"$MGCBIN" build
find .build -type f \( -name MagiCoreApp -o -name MagiCoreApp.exe \) -print -quit | grep -q .
