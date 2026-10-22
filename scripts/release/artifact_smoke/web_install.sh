#!/usr/bin/env bash
# Release operation owner. / Owner thao tác phát hành.
set -euo pipefail
mkdir -p lifecycle-test/web-project
cd lifecycle-test/web-project

# Create package.json
echo '{"name":"test-web","version":"1.0.0","dependencies":{"express":"^4.18.0"}}' > package.json

# Install with the explicit web core (NOT testing: create-web, test, build, run — the release-binary E2E covers the full lifecycle).
# Install với core web tường minh (full lifecycle do release-binary E2E phủ).
"$MGC_RELEASE_BINARY" --core web install

# Verify
if [ ! -d "node_modules/express" ]; then
  echo "express not installed"
  exit 1
fi

echo "Web install smoke test passed (explicit core)"
