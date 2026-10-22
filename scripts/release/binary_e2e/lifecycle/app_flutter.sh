#!/usr/bin/env bash
# Exercise the installed binary through the Flutter application lifecycle.
# Chạy binary đã cài qua vòng đời ứng dụng Flutter.
set -euo pipefail
if [[ "$RUNNER_OS" == "Windows" ]]; then MGCBIN="$HOME/.magicore-test/bin/mgc.exe"; else MGCBIN="$HOME/.magicore-test/bin/mgc"; fi
WORK="$(mktemp -d)"
cd "$WORK"
"$MGCBIN" create-app flutter test-flut
cd test-flut
# Flutter is only the compiler/test runtime here. MGC owns
# dependency resolution, materialization, and mgc.lock.
# Flutter chỉ là runtime compile/test; dependency do MGC quản lý.
"$MGCBIN" add-app http
"$MGCBIN" install
test -f mgc.lock
test ! -f pubspec.lock
"$MGCBIN" test -- --verbose
# `web` is the universal CI target (no Android SDK / Xcode).
# `web` là target CI phổ quát (không cần Android SDK / Xcode).
"$MGCBIN" build
test -d build/web
