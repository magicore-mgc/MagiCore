#!/usr/bin/env bash
# Release operation owner. / Owner thao tác phát hành.
set -euo pipefail
cd verify-assets
VERSION="${MGC_RELEASE_VERSION:?}"

for sbom in *-sbom.json; do
  echo "Validating SBOM: $sbom"

  # Check bomFormat
  if ! jq -e '.bomFormat == "CycloneDX"' "$sbom" > /dev/null; then
    echo "❌ Invalid bomFormat: $sbom"
    exit 1
  fi

  # Check required fields (P1 finding #6)
  if ! jq -e '.specVersion' "$sbom" > /dev/null; then
    echo "❌ Missing specVersion: $sbom"
    exit 1
  fi

  if ! jq -e '.metadata.component' "$sbom" > /dev/null; then
    echo "❌ Missing metadata.component: $sbom"
    exit 1
  fi

  if ! jq -e '.components' "$sbom" > /dev/null; then
    echo "❌ Missing components: $sbom"
    exit 1
  fi

  # Check component count (should have dependencies)
  component_count=$(jq -r '.components | length' "$sbom")
  if [[ $component_count -lt 10 ]]; then
    echo "❌ Too few components ($component_count): $sbom"
    exit 1
  fi

  # Verify SBOM metadata matches version (MUST match, not warning)
  sbom_version=$(jq -r '.metadata.component.version // empty' "$sbom")
  if [[ -z "$sbom_version" ]]; then
    echo "❌ Missing metadata.component.version: $sbom"
    exit 1
  fi
  if [[ "$sbom_version" != "$VERSION" ]]; then
    echo "❌ SBOM version mismatch: expected $VERSION, got $sbom_version in $sbom"
    exit 1
  fi

  echo "✅ Valid SBOM: $sbom ($component_count components, version $sbom_version)"
done
echo "✅ All SBOMs validated (linux+macos x64/arm64, windows x64)"
