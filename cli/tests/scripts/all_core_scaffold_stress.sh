#!/usr/bin/env bash
# All-Core Scaffold Stress Test — Phase 2
# Verify web/ai/app/lib create runtime + embedded/cache/registry/fallback order

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
MGC_BIN="${MGC_BIN:-$PROJECT_ROOT/target/debug/mgc}"
[[ -x "$MGC_BIN" ]] || { echo "FAIL: executable MGC_BIN is required" >&2; exit 1; }
MGC_BIN="$(cd "$(dirname "$MGC_BIN")" && pwd)/$(basename "$MGC_BIN")"
TEST_DIR="$(mktemp -d "${TMPDIR:-/tmp}/mgc-scaffold-stress.XXXXXX")"
TEST_HOME="$TEST_DIR/home"

# Setup isolated test environment — môi trường test cô lập
export HOME="$TEST_HOME"
export USERPROFILE="$TEST_HOME"
export MGC_DATA_DIR="$TEST_HOME/data"
export MGC_CACHE_DIR="$TEST_HOME/cache"
export MGC_TEMPLATES_DIR="$TEST_HOME/templates"
export MAGICORE_STORE_ROOT="$TEST_HOME/store"
unset MAGICORE_TEMPLATE_DIR

PASSED=0
TOTAL=0

echo "=== All-Core Scaffold Stress Test ==="
echo "Binary: $MGC_BIN"
echo "Test dir: $TEST_DIR"
echo "Test home: $TEST_HOME"
echo

# Cleanup — dọn dẹp
cleanup() {
    rm -rf "$TEST_DIR"
}
trap cleanup EXIT

mkdir -p "$TEST_DIR" "$TEST_HOME"
cd "$TEST_DIR"

run_test() {
    local name="$1"
    shift
    TOTAL=$((TOTAL + 1))
    echo "Test $TOTAL: $name"
    if "$@" >/dev/null 2>&1; then
        echo "✓ PASS"
        PASSED=$((PASSED + 1))
        return 0
    else
        echo "✗ FAIL"
        return 1
    fi
}

run_test_expect_output() {
    local name="$1"
    local expected="$2"
    shift 2
    TOTAL=$((TOTAL + 1))
    echo "Test $TOTAL: $name"
    local output
    local status=0
    output=$("$@" 2>&1) || status=$?
    if [[ "$status" -ne 0 ]] && grep -q "$expected" <<<"$output"; then
        echo "✓ PASS (found: '$expected')"
        PASSED=$((PASSED + 1))
        return 0
    fi
    echo "✗ FAIL (expected '$expected' not found)"
    echo "   Output: ${output:0:200}"
    return 1
}


verify_file() {
    local name="$1"
    local file="$2"
    TOTAL=$((TOTAL + 1))
    echo "Test $TOTAL: $name"
    if [ -f "$file" ]; then
        echo "✓ PASS (file exists: $file)"
        PASSED=$((PASSED + 1))
        return 0
    else
        echo "✗ FAIL (file missing: $file)"
        return 1
    fi
}

verify_dir() {
    local name="$1"
    local dir="$2"
    TOTAL=$((TOTAL + 1))
    echo "Test $TOTAL: $name"
    if [ -d "$dir" ]; then
        echo "✓ PASS (dir exists: $dir)"
        PASSED=$((PASSED + 1))
        return 0
    else
        echo "✗ FAIL (dir missing: $dir)"
        return 1
    fi
}

verify_content() {
    local name="$1"
    local file="$2"
    local pattern="$3"
    TOTAL=$((TOTAL + 1))
    echo "Test $TOTAL: $name"
    if [ -f "$file" ] && grep -q "$pattern" "$file"; then
        echo "✓ PASS (content match: $pattern)"
        PASSED=$((PASSED + 1))
        return 0
    else
        echo "✗ FAIL (content mismatch or file missing)"
        return 1
    fi
}

# === WEB CORE === — core web
echo
echo "=== WEB CORE TESTS ==="

run_test "web: create vanilla with TypeScript" \
    "$MGC_BIN" create-web vanilla test-web-vanilla --ts

verify_file "web/vanilla: index.html" "test-web-vanilla/index.html"
verify_file "web/vanilla: mgc.toml" "test-web-vanilla/mgc.toml"
verify_file "web/vanilla: .mgc.core marker" "test-web-vanilla/.mgc.core"
verify_content "web/vanilla: .mgc.core = web" "test-web-vanilla/.mgc.core" "^web$"
verify_content "web/vanilla: index.html contains project name" "test-web-vanilla/index.html" "test-web-vanilla"

# Test typo detection — kiểm tra typo detection
run_test_expect_output "web: typo detection (nextjs@laster)" "Did you mean" \
    "$MGC_BIN" create-web nextjs@laster test-typo

# === AI CORE === — core AI
echo
echo "=== AI CORE TESTS ==="

run_test "ai: create python-agent" \
    "$MGC_BIN" create-ai python-agent test-ai-python

verify_file "ai/python-agent: pyproject.toml" "test-ai-python/pyproject.toml"
verify_file "ai/python-agent: mgc.toml" "test-ai-python/mgc.toml"
verify_file "ai/python-agent: .mgc.core marker" "test-ai-python/.mgc.core"
verify_content "ai/python-agent: .mgc.core = ai" "test-ai-python/.mgc.core" "^ai$"
verify_file "ai/python-agent: models/ dir" "test-ai-python/models/README.md"
verify_dir "ai/python-agent: configs/ dir" "test-ai-python/configs"

# === APP CORE === — core app
echo
echo "=== APP CORE TESTS ==="

run_test "app: create flutter" \
    "$MGC_BIN" create-app flutter test-app-flutter

verify_file "app/flutter: pubspec.yaml" "test-app-flutter/pubspec.yaml"
verify_file "app/flutter: mgc.toml" "test-app-flutter/mgc.toml"
verify_file "app/flutter: .mgc.core marker" "test-app-flutter/.mgc.core"
verify_content "app/flutter: .mgc.core = app" "test-app-flutter/.mgc.core" "^app$"
verify_file "app/flutter: lib/main.dart" "test-app-flutter/lib/main.dart"

# === LIB CORE === — core lib
echo
echo "=== LIB CORE TESTS ==="

# Template refs are not compiler versions. — Ref template không phải version toolchain.
run_test "lib: create rust default" \
    "$MGC_BIN" create-lib rust test-lib-rust

verify_file "lib/rust: Cargo.toml" "test-lib-rust/Cargo.toml"
verify_file "lib/rust: src/lib.rs" "test-lib-rust/src/lib.rs"
verify_file "lib/rust: mgc.toml" "test-lib-rust/mgc.toml"
verify_file "lib/rust: .mgc.core marker" "test-lib-rust/.mgc.core"
verify_content "lib/rust: .mgc.core = lib" "test-lib-rust/.mgc.core" "^lib$"
verify_content "lib/rust: Cargo.toml has project name" "test-lib-rust/Cargo.toml" 'name = "test-lib-rust"'
verify_content "lib/rust: src/lib.rs has function" "test-lib-rust/src/lib.rs" "pub fn"

# Test other lib frameworks (fallback if no embedded) — test framework lib khác (fallback nếu không có embedded)
run_test "lib: create python (fallback)" \
    "$MGC_BIN" create-lib python test-lib-python

verify_file "lib/python: mgc.toml" "test-lib-python/mgc.toml"
verify_file "lib/python: .mgc.core marker" "test-lib-python/.mgc.core"

run_test "lib: create go (fallback)" \
    "$MGC_BIN" create-lib go test-lib-go

verify_file "lib/go: mgc.toml" "test-lib-go/mgc.toml"
verify_file "lib/go: .mgc.core marker" "test-lib-go/.mgc.core"

run_test "lib: create ts (fallback)" \
    "$MGC_BIN" create-lib ts test-lib-ts

verify_file "lib/ts: mgc.toml" "test-lib-ts/mgc.toml"
verify_file "lib/ts: .mgc.core marker" "test-lib-ts/.mgc.core"

# Verify cache content actually produced by the web runs, not manually written markers.
# Chỉ kiểm cache do CLI tạo; không cộng PASS cho biến env hay archive nằm trong source.
verify_file "web: real embedded template cache manifest" "$MGC_TEMPLATES_DIR/web/frontend/vanilla/template.toml"
verify_file "web: real embedded template cache source" "$MGC_TEMPLATES_DIR/web/frontend/vanilla/sources/index.html"

echo "=== Results ==="
echo "Passed: $PASSED/$TOTAL"

if [ "$PASSED" -eq "$TOTAL" ]; then
    echo "✓ ALL SCAFFOLD STRESS TESTS PASSED"
    echo
    echo "Summary:"
    echo "  web/vanilla: embedded kernel → project created"
    echo "  ai/python-agent: compiled processor → project created"
    echo "  app/flutter: compiled processor → project created"
    echo "  lib/rust: compiled processor → project created"
    echo "  lib/python,go,ts: fallback scaffold → project created"
    echo "  Typo detection: working"
    echo "  Cache path: hermetic (in test HOME)"
    echo "  Scope: scaffold checks only; install/build/test lifecycle not evaluated"
    exit 0
else
    FAILED=$((TOTAL - PASSED))
    echo "✗ $FAILED SCAFFOLD STRESS TESTS FAILED"
    exit 1
fi
