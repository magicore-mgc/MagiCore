#!/usr/bin/env bash
# npm package builder. / Owner dựng gói npm.
#
# Turns the release archives produced by the release workflow
# (magicore-<ver>-<os>-<arch>.tar.gz / .zip) into the five publishable
# npm/platforms/<platform>/ packages, each carrying the `mgc` binary.
# Idempotent: running it twice leaves byte-identical package trees, and it
# never creates a platform folder that has no binary (fail-closed).
#
# Biến archive phát hành thành 5 gói nền tảng chứa binary `mgc`. Chạy lại
# cho kết quả y hệt; thiếu archive thì dừng ngay, không để lại gói rỗng.
#
# Usage / Cách dùng:
#   bash scripts/release/build-npm-packages.sh --artifacts <dir> [--version <ver>]
#
# Exit codes / Mã thoát:
#   0  every package built
#   1  a package could not be built (bad archive, unsupported platform)
#   2  invalid usage
#   3  environment/precondition problem (missing tool, missing archive dir)

set -euo pipefail

readonly EXIT_USAGE=2
readonly EXIT_PRECONDITION=3

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ARTIFACTS=""
VERSION=""

usage() {
  cat <<'EOF'
Usage: build-npm-packages.sh --artifacts <dir> [--version <version>]

  --artifacts <dir>  Directory holding the release archives
                     (magicore-<ver>-<os>-<arch>.tar.gz / .zip).
  --version <ver>    Release version. Defaults to [workspace.package] in Cargo.toml.

Every package is written to npm/platforms/<platform>/ with the binary as
`mgc` (`mgc.exe` on win32) and the repository LICENSE.
EOF
}

die() {
  local code="$1"
  shift
  printf 'build-npm-packages: %s\n' "$*" >&2
  exit "$code"
}

require_value() {
  local flag="$1" value="${2:-}"
  if [[ -z "$value" || "$value" == --* ]]; then
    die "$EXIT_USAGE" "$flag requires a value (run with --help for usage)"
  fi
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --artifacts) require_value "$1" "${2:-}"; ARTIFACTS="$2"; shift 2 ;;
    --version) require_value "$1" "${2:-}"; VERSION="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) die "$EXIT_USAGE" "unknown argument '$1' (run with --help for usage)" ;;
  esac
done

[[ -n "$ARTIFACTS" ]] || die "$EXIT_USAGE" "--artifacts <dir> is required (run with --help for usage)"
[[ -d "$ARTIFACTS" ]] || die "$EXIT_PRECONDITION" "artifact directory not found: $ARTIFACTS"
[[ -d "$ROOT/npm/platforms" ]] || die "$EXIT_PRECONDITION" "npm/platforms is missing from $ROOT (is this the MagiCore repository root?)"

if [[ -z "$VERSION" ]]; then
  VERSION="$(sed -n '/^\[workspace.package\]/,/^\[/s/^version = "\([^"]*\)"/\1/p' "$ROOT/Cargo.toml" | head -1)"
fi
[[ -n "$VERSION" ]] || die "$EXIT_PRECONDITION" "cannot read the version from Cargo.toml; pass --version <version>"
if [[ ! "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?$ ]]; then
  die "$EXIT_USAGE" "invalid version '$VERSION' (expected e.g. 1.2.0 or 1.2.0-rc.1)"
fi

# Release archive names use release platform labels: linux | macos | windows.
# npm os fields use node's process.platform: linux | darwin | win32. The
# mapping below is the only place the two vocabularies meet, so a new platform
# is added in one row.
# Tên archive dùng nhãn phát hành (macos/windows), npm dùng process.platform
# (darwin/win32). Bảng dưới là nơi duy nhất hai bộ tên gặp nhau.
readonly -a PLATFORMS=(
  "linux-x64   linux   x64   tar.gz mgc"
  "linux-arm64 linux   arm64 tar.gz mgc"
  "darwin-x64  macos   x64   tar.gz mgc"
  "darwin-arm64 macos  arm64 tar.gz mgc"
  "win32-x64   windows x64   zip    mgc.exe"
)

command -v python3 >/dev/null 2>&1 \
  || die "$EXIT_PRECONDITION" "python3 is required to rewrite package.json (install Python 3 and retry)"

# Pre-flight: every archive and its binary must exist before any package is
# rewritten, so a partial run can never leave a half-built tree.
# Kiểm đủ archive + binary trước khi ghi, tránh để lại cây gói dở dang.
work="$(mktemp -d "${TMPDIR:-/tmp}/mgc-npm.XXXXXX")"
trap 'rm -rf "$work"' EXIT

extracted=()
for row in "${PLATFORMS[@]}"; do
  read -r platform release_os arch ext binary <<<"$row"
  archive="$ARTIFACTS/magicore-$VERSION-$release_os-$arch.$ext"

  if [[ ! -f "$archive" ]]; then
    die 1 "missing release archive: $archive
  The release workflow must build magicore-$release_os-$arch.$ext first.
  Check the artifact folder passed to --artifacts."
  fi

  out="$work/$platform"
  mkdir -p "$out"

  case "$ext" in
    tar.gz)
      # Gzipped tar: extract only the named member, keeping the layout the
      # release packager writes (binary at the archive root). No
      # --strip-components: stripping would leave nothing at that depth.
      # tar: chỉ giải nén đúng file, giữ vị trí gốc trong archive.
      tar -xzf "$archive" -C "$out" "$binary" \
        || die 1 "archive $archive does not contain '$binary' at its root"
      ;;
    zip)
      # Windows run this on Linux (ubuntu-latest); unzip is standard there.
      # zip: chạy trên Linux, `unzip` có sẵn.
      command -v unzip >/dev/null 2>&1 \
        || die "$EXIT_PRECONDITION" "unzip is required to read $archive (install unzip and retry)"
      unzip -q -o "$archive" "$binary" -d "$out" \
        || die 1 "archive $archive does not contain '$binary' at its root"
      ;;
    *)
      die 1 "unsupported archive extension '$ext' for $platform"
      ;;
  esac

  [[ -s "$out/$binary" ]] \
    || die 1 "'$binary' extracted from $archive is empty — rebuild the release archive for $platform"
  extracted+=("$platform:$binary:$out/$binary")
done

# Write phase: only reached when every archive above validated.
# Pha ghi: chỉ chạy khi mọi archive đã hợp lệ.
for entry in "${extracted[@]}"; do
  IFS=: read -r platform binary source <<<"$entry"
  dest="$ROOT/npm/platforms/$platform"
  [[ -d "$dest" ]] || die 1 "npm/platforms/$platform does not exist; add its package.json before releasing"

  # Replace (not patch) the binary so a stale build never survives a rerun.
  # Thay hẳn binary để build cũ không sót lại khi chạy lại.
  rm -f "$dest/mgc" "$dest/mgc.exe"
  cp "$source" "$dest/$binary"
  chmod 0755 "$dest/$binary"
  cp "$ROOT/LICENSE" "$dest/LICENSE"

  # Set the version in this package and, when the file declares it, its own
  # optionalDependencies — anything that points at another @magicore package is
  # pinned to the same exact version, so a wrapper can never resolve a binary
  # from a different release.
  # Gán version cho gói; mọi optionalDependency trỏ tới @magicore/* được ghim
  # đúng version này để wrapper không lẫn binary khác bản.
  PACKAGE_JSON="$dest/package.json" NPM_VERSION="$VERSION" python3 - <<'PY'
import json
import os
import pathlib

path = pathlib.Path(os.environ["PACKAGE_JSON"])
version = os.environ["NPM_VERSION"]
data = json.loads(path.read_text(encoding="utf-8"))

data["version"] = version
for field in ("dependencies", "optionalDependencies", "peerDependencies"):
    deps = data.get(field)
    if not isinstance(deps, dict):
        continue
    for name, spec in list(deps.items()):
        if name.startswith("@magicore/") and spec.startswith("="):
            deps[name] = f"={version}"

path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
PY

  printf 'built %s -> npm/platforms/%s/%s (%s bytes, v%s)\n' \
    "$(basename "$source")" "$platform" "$binary" "$(wc -c <"$dest/$binary" | tr -d ' ')" "$VERSION"
done

# The wrapper is versioned here too; its name/optionalDependencies are owned
# by the reviewed npm/cli/package.json (pinned with "=", updated above).
# Wrapper cũng lấy version tại đây; tên/phụ thuộc do package.json đã duyệt giữ.
PACKAGE_JSON="$ROOT/npm/cli/package.json" NPM_VERSION="$VERSION" python3 - <<'PY'
import json
import os
import pathlib

path = pathlib.Path(os.environ["PACKAGE_JSON"])
data = json.loads(path.read_text(encoding="utf-8"))
data["version"] = os.environ["NPM_VERSION"]
deps = data.get("optionalDependencies")
if not isinstance(deps, dict) or not deps:
    raise SystemExit("npm/cli/package.json must declare optionalDependencies")
for name in deps:
    deps[name] = f"={os.environ['NPM_VERSION']}"
path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
print(f"built wrapper @magicore/cli v{os.environ['NPM_VERSION']} for {len(deps)} platforms")
PY
