#!/usr/bin/env bash
# Exercise the supplied CLI binary; report only operations executed in this run.
# Kiểm binary thật và filesystem thật; không cộng PASS từ file nguồn hay test chưa chạy.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
MGC_BIN="${MGC_BIN:-$PROJECT_ROOT/target/debug/mgc}"
[[ -x "$MGC_BIN" ]] || { echo "FAIL: executable MGC_BIN is required" >&2; exit 1; }
MGC_BIN="$(cd "$(dirname "$MGC_BIN")" && pwd)/$(basename "$MGC_BIN")"
TEST_DIR="$(mktemp -d "${TMPDIR:-/tmp}/mgc-acceptance.XXXXXX")"
trap 'rm -rf "$TEST_DIR"' EXIT
export HOME="$TEST_DIR/home"
export USERPROFILE="$HOME"
export MGC_DATA_DIR="$HOME/data"
export MGC_CACHE_DIR="$HOME/cache"
export MGC_TEMPLATES_DIR="$HOME/templates"
export MAGICORE_STORE_ROOT="$HOME/store"
unset MAGICORE_TEMPLATE_DIR
mkdir -p "$HOME"
cd "$TEST_DIR"

PASSED=0
run_test() {
    local name="$1"
    shift
    if "$@"; then
        PASSED=$((PASSED + 1))
        echo "PASS: $name"
    else
        echo "FAIL: $name" >&2
        exit 1
    fi
}
reject_typo() {
    local output
    if output=$("$MGC_BIN" create-web nextjs@laster typo-project 2>&1); then
        echo "FAIL: invalid ref unexpectedly succeeded" >&2
        return 1
    fi
    [[ "$output" == *"Did you mean"* ]] && [[ ! -e typo-project ]]
}
check_project() {
    local project="$1" core="$2" manifest="$3" source="$4"
    [[ -s "$project/$manifest" && -s "$project/$source" && -s "$project/mgc.toml" ]] &&
    [[ "$(tr -d '\r\n' < "$project/.mgc.core")" == "$core" ]]
}

run_test "binary executes" "$MGC_BIN" --version
run_test "typo fails without creating project" reject_typo
run_test "web default scaffold" "$MGC_BIN" create-web vanilla test-web --ts
run_test "web manifest/source/identity" check_project test-web web package.json index.html
run_test "AI default scaffold" "$MGC_BIN" create-ai python-agent test-ai
run_test "AI manifest/source/identity" check_project test-ai ai pyproject.toml src/agent.py
run_test "App default scaffold" "$MGC_BIN" create-app flutter test-app
run_test "App manifest/source/identity" check_project test-app app pubspec.yaml lib/main.dart
run_test "Lib default scaffold" "$MGC_BIN" create-lib rust test-lib
run_test "Lib manifest/source/identity" check_project test-lib lib Cargo.toml src/lib.rs
run_test "second project with shared template cache" "$MGC_BIN" create-web vanilla test-web-again --ts
run_test "second project's manifest/source/identity" check_project test-web-again web package.json index.html
printf 'PASS: %s executed scaffold checks; package install/build/test lifecycle was not evaluated\n' "$PASSED"
