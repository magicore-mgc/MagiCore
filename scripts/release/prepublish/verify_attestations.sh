#!/usr/bin/env bash
# Release operation owner. / Owner thao tác phát hành.
set -euo pipefail
cd verify-assets
FAILED=0
for archive in *.tar.gz *.zip; do
  if [[ -f "$archive" ]]; then
    echo "Verifying attestation: $archive"
    # Use exit code, not grep string (P1 finding #5)
    if gh attestation verify "$archive" --owner "$MGC_RELEASE_REPOSITORY_OWNER"; then
      echo "✅ Attestation verified: $archive"
    else
      echo "❌ Attestation FAILED: $archive"
      FAILED=$((FAILED + 1))
    fi
  fi
done

if [[ $FAILED -gt 0 ]]; then
  echo "❌ $FAILED attestations failed - BLOCKING RELEASE"
  exit 1
fi
echo "✅ All attestations verified"
