#!/usr/bin/env bash
# Package both Unix CLI variants and write their SHA-256 sidecars.
set -euo pipefail

artifact="${MGC_RELEASE_ARTIFACT_NAME:?MGC_RELEASE_ARTIFACT_NAME is required}"
web_artifact="${MGC_RELEASE_WEB_ARTIFACT_NAME:?MGC_RELEASE_WEB_ARTIFACT_NAME is required}"
tar -C dist/magicore -czvf "$artifact" mgc
shasum -a 256 "$artifact" > "${artifact}.sha256"
tar -C dist/magicore-web -czvf "$web_artifact" mgc
shasum -a 256 "$web_artifact" > "${web_artifact}.sha256"
