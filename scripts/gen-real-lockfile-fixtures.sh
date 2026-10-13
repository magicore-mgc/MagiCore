#!/usr/bin/env bash
# Regenerate the REAL-lockfile fixtures for mgc-lockfile import tests.
# Sinh lại fixture lockfile THẬT cho test import của mgc-lockfile.
#
# P0 finding #9 (2026-09-12): hand-written fixtures pass even when the
# importer drifts from what real writers emit. These fixtures are
# produced by the PINNED tools below and committed; when the tool
# versions change, regenerate and review the diff — the test then
# catches writer-shape drift.
# P0 finding #9: fixture viết tay pass ngay cả khi importer trôi khỏi
# output thật của writer. Fixture này do CÔNG CỤ GHIM bên dưới sinh ra
# và được commit; khi đổi version công cụ, sinh lại và review diff —
# test sẽ bắt được sự trôi của writer.
#
# Usage: scripts/gen-real-lockfile-fixtures.sh <output-dir>
# Requires: bun 1.4.3, pnpm 12.12.0, deno 2.9.7 (pinned — do not bump silently)
set -euo pipefail

BUN_VERSION="1.4.3"    # pinned — ghim
PNPM_VERSION="12.12.0" # pinned — ghim
DENO_VERSION="2.9.7"   # pinned — ghim
OUT="${1:?usage: gen-real-lockfile-fixtures.sh <output-dir>}"

check_version() {
  local tool="$1" expected="$2" actual version_output
  command -v "$tool" >/dev/null 2>&1 || {
    echo "required tool not found: $tool $expected" >&2
    exit 1
  }
  case "$tool" in
    bun) actual="$(bun --version)" ;;
    pnpm) actual="$(pnpm --version)" ;;
    deno)
      version_output="$(deno --version)"
      actual="${version_output#deno }"
      actual="${actual%% *}"
      ;;
    *) echo "unsupported fixture tool: $tool" >&2; exit 1 ;;
  esac
  [[ "$actual" == "$expected" ]] || {
    echo "$tool version mismatch: expected $expected, got $actual" >&2
    exit 1
  }
}

check_version bun "$BUN_VERSION"
check_version pnpm "$PNPM_VERSION"
check_version deno "$DENO_VERSION"

mkdir -p "$OUT"
TMP_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMP_ROOT"' EXIT
mkdir -p "$TMP_ROOT/home" "$TMP_ROOT/bun-cache" "$TMP_ROOT/pnpm-store" "$TMP_ROOT/deno-cache"

# --- bun.lock: real `bun install` output (JSONC writer, v2 schema) ---
WORK="$TMP_ROOT/bun"
mkdir -p "$WORK"
cat > "$WORK/package.json" <<'JSON'
{
  "name": "mgc-fixture",
  "dependencies": {
    "debug": "4.4.3"
  },
  "devDependencies": {
    "left-pad": "1.3.0"
  }
}
JSON
(cd "$WORK" && HOME="$TMP_ROOT/home" BUN_INSTALL_CACHE_DIR="$TMP_ROOT/bun-cache" bun install --lockfile-only --ignore-scripts --silent)
cp "$WORK/bun.lock" "$OUT/bun.lock"

# --- pnpm-lock.yaml: real `pnpm install` output (lockfileVersion 9.0) ---
WORK="$TMP_ROOT/pnpm"
mkdir -p "$WORK"
cat > "$WORK/package.json" <<'JSON'
{
  "name": "mgc-fixture",
  "dependencies": {
    "ms": "2.1.3"
  },
  "devDependencies": {
    "left-pad": "1.3.0"
  }
}
JSON
(cd "$WORK" && HOME="$TMP_ROOT/home" PNPM_STORE_DIR="$TMP_ROOT/pnpm-store" pnpm install --lockfile-only --ignore-scripts --silent)
cp "$WORK/pnpm-lock.yaml" "$OUT/pnpm-lock.yaml.fixture"

# --- deno.lock: real `deno cache` output (v5 schema, jsr + npm pins) ---
WORK="$TMP_ROOT/deno"
mkdir -p "$WORK"
cat > "$WORK/deno.json" <<'JSON'
{
  "imports": {
    "lodash": "npm:lodash@4.17.20",
    "@std/bytes": "jsr:@std/bytes@0.224.0"
  }
}
JSON
cat > "$WORK/main.ts" <<'TS'
import lodash from "lodash";
import { concat } from "@std/bytes";
console.log(lodash.VERSION, concat(new Uint8Array([1]), new Uint8Array([2])).length);
TS
(cd "$WORK" && HOME="$TMP_ROOT/home" DENO_DIR="$TMP_ROOT/deno-cache" deno cache main.ts)
cp "$WORK/deno.lock" "$OUT/deno.lock"

echo "fixtures written to $OUT (bun $BUN_VERSION, pnpm $PNPM_VERSION, deno $DENO_VERSION)"
