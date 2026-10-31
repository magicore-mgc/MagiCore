#!/usr/bin/env bash
# npm package contract tests. / Kiểm thử hợp đồng gói npm.
#
# Offline only: parses the manifests, cross-checks versions, checks the
# wrapper's dependency set and the launcher shim. Never publishes, never
# touches the network, never writes to the package tree.
# Chỉ chạy offline: parse manifest, đối chiếu version, kiểm phụ thuộc của
# wrapper và file shim. Không publish, không gọi mạng, không ghi vào gói.
#
# Usage / Cách dùng:
#   bash scripts/test/test_npm_packages.sh
#
# Exit: 0 all checks pass, 1 at least one check failed.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
readonly ROOT
readonly NPM_DIR="$ROOT/npm"
readonly WRAPPER="$NPM_DIR/cli/package.json"
readonly SHIM="$NPM_DIR/cli/bin/mgc.js"

# The five published platform packages; the wrapper must declare exactly these.
# Năm gói nền tảng phát hành; wrapper phải khai báo đúng tập này.
readonly -a PLATFORMS=(
  "linux-x64"
  "linux-arm64"
  "darwin-x64"
  "darwin-arm64"
  "win32-x64"
)
readonly EXPECTED_OPTIONAL=(
  "@magicore/cli-linux-x64"
  "@magicore/cli-linux-arm64"
  "@magicore/cli-darwin-x64"
  "@magicore/cli-darwin-arm64"
  "@magicore/cli-win32-x64"
)

failures=0
checks=0

pass() {
  checks=$((checks + 1))
  printf '  ok   %s\n' "$1"
}

fail() {
  checks=$((checks + 1))
  failures=$((failures + 1))
  printf '  FAIL %s\n' "$1" >&2
}

check() {
  # check <description> <command...>
  local description="$1"
  shift
  if "$@" >/dev/null 2>&1; then pass "$description"; else fail "$description"; fi
}

check_file() {
  if [[ -f "$1" ]]; then pass "$2"; else fail "$2 (missing: $1)"; fi
}

command -v python3 >/dev/null 2>&1 || {
  printf 'test_npm_packages: python3 is required to parse package.json\n' >&2
  exit 1
}

printf 'npm package contract\n'
printf 'root: %s\n\n' "$NPM_DIR"

# ---------------------------------------------------------------------------
# 1. Every manifest parses and names the expected package.
# Mỗi manifest parse được và đúng tên gói.
# ---------------------------------------------------------------------------
printf 'manifests\n'
check_file "$WRAPPER" "wrapper package.json exists"
check_file "$NPM_DIR/cli/README.md" "wrapper README.md exists"
check_file "$NPM_DIR/cli/LICENSE" "wrapper LICENSE exists"

for platform in "${PLATFORMS[@]}"; do
  check_file "$NPM_DIR/platforms/$platform/package.json" "platform $platform package.json exists"
done

if [[ -f "$WRAPPER" ]]; then
  if python3 -c 'import json,sys; json.load(open(sys.argv[1]))' "$WRAPPER" 2>/dev/null; then
    pass "wrapper package.json is valid JSON"
  else
    fail "wrapper package.json is valid JSON"
  fi
fi

for platform in "${PLATFORMS[@]}"; do
  manifest="$NPM_DIR/platforms/$platform/package.json"
  [[ -f "$manifest" ]] || continue
  if python3 -c 'import json,sys; json.load(open(sys.argv[1]))' "$manifest" 2>/dev/null; then
    pass "platform $platform package.json is valid JSON"
  else
    fail "platform $platform package.json is valid JSON"
  fi
done

# ---------------------------------------------------------------------------
# 2. Semantic contract, checked in one Python pass so a failure reports every
#    offending field instead of stopping at the first.
# Hợp đồng ngữ nghĩa kiểm trong một lượt Python; báo hết lỗi chứ không dừng ở
# lỗi đầu.
# ---------------------------------------------------------------------------
printf '\ncontract\n'
contract_script="$(mktemp "${TMPDIR:-/tmp}/mgc-npm-contract.XXXXXX")"
trap 'rm -f "$contract_script"' EXIT
cat >"$contract_script" <<'PY'
import json
import os
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
npm = root / "npm"
platforms = sys.argv[2:]
problems = []

def load(path):
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError:
        problems.append(f"{path.relative_to(root)}: missing")
    except json.JSONDecodeError as error:
        problems.append(f"{path.relative_to(root)}: invalid JSON ({error})")
    return None

def expect(name, condition, message):
    if not condition:
        problems.append(f"{name}: {message}")

# Cargo.toml is the release-version source of truth.
cargo_version = None
in_workspace = False
for line in (root / "Cargo.toml").read_text(encoding="utf-8").splitlines():
    if line.strip() == "[workspace.package]":
        in_workspace = True
        continue
    if in_workspace:
        if line.startswith("["):
            break
        if line.startswith("version"):
            cargo_version = line.split('"')[1]
            break
expect("Cargo.toml", cargo_version, "cannot read [workspace.package] version")

wrapper = load(npm / "cli" / "package.json")
wrapper_version = None

if wrapper is not None:
    name = "@magicore/cli"
    expect(name, wrapper.get("name") == name, f"name must be '{name}'")
    wrapper_version = wrapper.get("version")
    expect(name, wrapper_version == cargo_version,
           f"version {wrapper_version!r} must match Cargo.toml {cargo_version!r}")

    expect(name, wrapper.get("bin") == {"mgc": "bin/mgc.js", "magicore": "bin/mgc.js"},
           "bin must map both 'mgc' and 'magicore' to bin/mgc.js")
    expect(name, wrapper.get("engines", {}).get("node") == ">=18",
           "engines.node must be '>=18'")
    expect(name, wrapper.get("files") == ["bin/", "README.md", "LICENSE"],
           "files must be exactly ['bin/', 'README.md', 'LICENSE']")
    expect(name, wrapper.get("publishConfig") == {"access": "public", "provenance": True},
           "publishConfig must be {access: public, provenance: true}")
    expect(name, wrapper.get("license") == "MIT", "license must be MIT")
    expect(name, wrapper.get("repository", {}).get("url") == "https://github.com/mingd-153/MagiCore",
           "repository.url must be https://github.com/mingd-153/MagiCore")
    expect(name, "dependencies" not in wrapper,
           "the wrapper must have no runtime dependencies (a dependency would slow every startup)")
    expect(name, "type" not in wrapper or wrapper.get("type") == "commonjs",
           "the shim is CommonJS; do not set a module type")

    optional = wrapper.get("optionalDependencies", {})
    expected = {f"@magicore/cli-{p}" for p in platforms}
    expect(name, isinstance(optional, dict) and set(optional) == expected,
           f"optionalDependencies must be exactly the 5 platform packages; "
           f"missing={sorted(expected - set(optional))} extra={sorted(set(optional) - expected)}")
    for dep, spec in sorted(optional.items()):
        if wrapper_version is not None:
            expect(name, spec == f"={wrapper_version}",
                   f"optionalDependencies['{dep}'] must be pinned '={wrapper_version}', got {spec!r}")

for platform in platforms:
    package = f"@magicore/cli-{platform}"
    manifest = load(npm / "platforms" / platform / "package.json")
    if manifest is None:
        continue
    expect(package, manifest.get("name") == package, f"name must be '{package}'")
    version = manifest.get("version")
    expect(package, version == cargo_version,
           f"version {version!r} must match Cargo.toml {cargo_version!r}")
    if wrapper_version is not None:
        expect(package, version == wrapper_version,
               f"version {version!r} must match the wrapper {wrapper_version!r}")
    release_os, arch = platform.rsplit("-", 1)
    expect(package, manifest.get("os") == [release_os], f"os must be ['{release_os}']")
    expect(package, manifest.get("cpu") == [arch], f"cpu must be ['{arch}']")
    expect(package, manifest.get("files") == ["mgc", "mgc.exe", "LICENSE"],
           "files must be ['mgc', 'mgc.exe', 'LICENSE']")
    expect(package, manifest.get("publishConfig", {}).get("access") == "public",
           "publishConfig.access must be public (scoped packages are private by default)")

if problems:
    print("\n".join(problems))
    sys.exit(1)
PY

contract_output=""
if contract_output="$(python3 "$contract_script" "$ROOT" "${PLATFORMS[@]}" 2>&1)"; then
  pass "manifest contract (names, versions, os/cpu, publishConfig, pinned optionalDependencies)"
else
  while IFS= read -r problem; do
    [[ -n "$problem" ]] && fail "$problem"
  done <<<"$contract_output"
fi

# ---------------------------------------------------------------------------
# 3. Version consistency stated once, as an explicit assertion.
# Đối chiếu version thành một assert riêng.
# ---------------------------------------------------------------------------
printf '\nversion consistency\n'
if [[ -f "$WRAPPER" ]]; then
  versions="$(python3 - "$NPM_DIR" "${PLATFORMS[@]}" <<'PY'
import json
import pathlib
import sys

npm = pathlib.Path(sys.argv[1])
paths = [npm / "cli" / "package.json"] + [
    npm / "platforms" / platform / "package.json" for platform in sys.argv[2:]
]
for path in paths:
    try:
        print(f"{json.loads(path.read_text(encoding='utf-8')).get('version', '<none>')} {path}")
    except (FileNotFoundError, json.JSONDecodeError):
        print(f"<unreadable> {path}")
PY
)"
  unique="$(awk '{print $1}' <<<"$versions" | sort -u)"
  if [[ "$(wc -l <<<"$unique" | tr -d ' ')" -eq 1 && "$unique" == "1.2.0" ]]; then
    pass "all 6 packages are version 1.2.0"
  else
    fail "versions differ or are not 1.2.0:"
    sed 's/^/        /' <<<"$versions" >&2
  fi
fi

# ---------------------------------------------------------------------------
# 4. Launcher shim.
# File shim.
# ---------------------------------------------------------------------------
printf '\nlauncher shim\n'
check_file "$SHIM" "bin/mgc.js exists"
if [[ -f "$SHIM" ]]; then
  check "bin/mgc.js has a node shebang" grep -q '^#!/usr/bin/env node' "$SHIM"
  check "bin/mgc.js resolves the platform package via require.resolve" \
    grep -q "require.resolve(\`\${pkgName}/package.json\`)" "$SHIM"
  check "bin/mgc.js spawns with inherited stdio" \
    grep -q "stdio: 'inherit'" "$SHIM"
  check "bin/mgc.js forwards every user argument (process.argv.slice(2))" \
    grep -q 'process.argv.slice(2)' "$SHIM"
  check "bin/mgc.js exits with the child's status" grep -q 'process.exit(result.status' "$SHIM"
  check "bin/mgc.js downloads nothing at runtime" \
    bash -c '! grep -Eq "https?\.get|https?\.request|node:https|require\(.node:https.\)|child_process.*(curl|wget)|fetch\(" "$1"' _ "$SHIM"
  if [[ -x "$SHIM" ]]; then
    pass "bin/mgc.js is executable"
  else
    fail "bin/mgc.js is executable (chmod +x npm/cli/bin/mgc.js)"
  fi
  # npm sets the bin executable itself, but a package-lock/registry round-trip
  # keeps the mode from git — pin it so a fresh checkout stays 0755.
  # npm tự set quyền bin, nhưng giữ mode trong git để checkout mới vẫn chạy được.
  if git -C "$ROOT" ls-files -s -- npm/cli/bin/mgc.js 2>/dev/null | grep -q '^100755'; then
    pass "bin/mgc.js is tracked with mode 100755"
  elif git -C "$ROOT" ls-files --error-unmatch npm/cli/bin/mgc.js >/dev/null 2>&1; then
    fail "bin/mgc.js is tracked with mode 100755 (git update-index --chmod=+x npm/cli/bin/mgc.js)"
  else
    printf '  skip bin/mgc.js git mode (not tracked yet)\n'
  fi
fi

[[ "$failures" == "0" ]] || { printf '\n%d checks, %d failed\n' "$checks" "$failures"; exit 1; }
printf '%d checks, 0 failed\n' "$checks"
