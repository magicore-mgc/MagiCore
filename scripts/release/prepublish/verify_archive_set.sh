#!/usr/bin/env bash
# Release operation owner. / Owner thao tác phát hành.
set -euo pipefail
VERSION="${MGC_RELEASE_VERSION:?}"

# Generate expected files via contract (single source of truth)
EXPECTED_FILES=()

for os in linux macos windows; do
  for arch in x64 arm64; do
    # Windows publishes x64 only; Linux and macOS publish x64 and arm64.
    # Windows chỉ phát hành x64; Linux và macOS phát hành cả x64 lẫn arm64.
    for variant in all web; do
      names=$(./scripts/release-artifact-contract.sh "$VERSION" "$os" "$arch" "$variant")
      archive=$(echo "$names" | grep "^ARCHIVE=" | cut -d= -f2)
      checksum=$(echo "$names" | grep "^CHECKSUM=" | cut -d= -f2)
      sbom=$(echo "$names" | grep "^SBOM=" | cut -d= -f2)

      EXPECTED_FILES+=("$archive")
      EXPECTED_FILES+=("$checksum")
      EXPECTED_FILES+=("$sbom")
    done
  done
done

cd verify-assets
MISSING=0
PRESENT_ARM64_MACOS=0
PRESENT_ARM64_LINUX=0
for file in "${EXPECTED_FILES[@]}"; do
  # Windows ARM64 is not in the supported release matrix; Linux/macOS ARM64 are required.
  # Windows ARM64 chưa thuộc ma trận phát hành; Linux/macOS ARM64 là bắt buộc.
  case "$file" in
    magicore-*-windows-arm64*)
      [[ -f "$file" ]] && echo "BONUS present: $file"
      continue
      ;;
    magicore-*-linux-arm64*)
      if [[ -f "$file" ]]; then
        PRESENT_ARM64_LINUX=$((PRESENT_ARM64_LINUX + 1))
      else
        echo "MISSING REQUIRED (linux arm64): $file"
        MISSING=$((MISSING + 1))
      fi
      continue
      ;;
    magicore-*-macos-arm64*)
      if [[ -f "$file" ]]; then
        PRESENT_ARM64_MACOS=$((PRESENT_ARM64_MACOS + 1))
      else
        echo "MISSING REQUIRED (macos arm64, item 12): $file"
        MISSING=$((MISSING + 1))
      fi
      continue
      ;;
  esac
  if [[ ! -f "$file" ]]; then
    echo "MISSING: $file"
    MISSING=$((MISSING + 1))
  else
    echo "Found: $file"
  fi
done

if [[ $MISSING -gt 0 ]]; then
  echo "$MISSING files missing - BLOCKING RELEASE"
  exit 1
fi
if [[ $PRESENT_ARM64_MACOS -lt 6 ]]; then
  echo "macOS arm64 artifact set incomplete (expected 6 files: archives+checksums+SBOMs for all/web, got $PRESENT_ARM64_MACOS)" >&2
  exit 1
fi
if [[ $PRESENT_ARM64_LINUX -lt 6 ]]; then
  echo "Linux arm64 artifact set incomplete (expected 6 files: archives+checksums+SBOMs for all/web, got $PRESENT_ARM64_LINUX)" >&2
  exit 1
fi
echo "All 10 required archives + checksums + SBOMs present (incl. Linux and macOS arm64)"
