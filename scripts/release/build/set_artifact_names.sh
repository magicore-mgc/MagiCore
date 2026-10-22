#!/usr/bin/env bash
# Release operation owner. / Owner thao tác phát hành.
set -euo pipefail
VERSION="${MGC_RELEASE_VERSION:?}"

# Use contract script as single source of truth
ALL_NAMES=$(./scripts/release-artifact-contract.sh "$VERSION" "$MGC_RELEASE_OS_NAME" "$MGC_RELEASE_ARCH" "all")
WEB_NAMES=$(./scripts/release-artifact-contract.sh "$VERSION" "$MGC_RELEASE_OS_NAME" "$MGC_RELEASE_ARCH" "web")

ARTIFACT_NAME=$(echo "$ALL_NAMES" | grep "^ARCHIVE=" | cut -d= -f2)
WEB_ARTIFACT_NAME=$(echo "$WEB_NAMES" | grep "^ARCHIVE=" | cut -d= -f2)

echo "artifact_name=$ARTIFACT_NAME" >> $GITHUB_OUTPUT
echo "web_artifact_name=$WEB_ARTIFACT_NAME" >> $GITHUB_OUTPUT
echo "Building all-core: $ARTIFACT_NAME"
echo "Building web-only: $WEB_ARTIFACT_NAME"
