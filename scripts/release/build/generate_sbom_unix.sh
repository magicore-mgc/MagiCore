#!/usr/bin/env bash
# Release operation owner. / Owner thao tác phát hành.
set -euo pipefail
cargo install cargo-cyclonedx --version 0.5.9 --locked

# Generate for all-features binary (matches build flags exactly)
cd cli
cargo cyclonedx \
  --format json \
  --spec-version 1.4 \
  --no-default-features \
  --features all \
  --target $MGC_RELEASE_TARGET
cd ..
mv cli/mgc.cdx.json "magicore-$MGC_RELEASE_VERSION-$MGC_RELEASE_OS_NAME-$MGC_RELEASE_ARCH-sbom.json"

# Generate for web-only binary (matches build flags exactly)
cd cli
cargo cyclonedx \
  --format json \
  --spec-version 1.4 \
  --no-default-features \
  --features web \
  --target $MGC_RELEASE_TARGET
cd ..
mv cli/mgc.cdx.json "magicore-web-$MGC_RELEASE_VERSION-$MGC_RELEASE_OS_NAME-$MGC_RELEASE_ARCH-sbom.json"

# Verify SBOMs exist and are valid CycloneDX
for sbom in *-sbom.json; do
  if ! jq -e '.bomFormat == "CycloneDX"' "$sbom" > /dev/null; then
    echo "❌ Invalid SBOM: $sbom"
    exit 1
  fi
  # Check required fields (P1 finding #6)
  if ! jq -e '.specVersion and .components and .metadata' "$sbom" > /dev/null; then
    echo "❌ SBOM missing required fields: $sbom"
    exit 1
  fi
  component_count=$(jq -r '.components | length' "$sbom")
  echo "✅ Valid SBOM: $sbom ($component_count components)"
done
