#!/usr/bin/env bash
set -euo pipefail

profile="release"
target=""
target_is_set=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --target)
      if [[ $# -lt 2 || -z "${2:-}" ]]; then
        echo "--target requires a non-empty value" >&2
        exit 2
      fi
      if [[ "$target_is_set" -eq 1 ]]; then
        echo "--target may only be provided once" >&2
        exit 2
      fi
      target="$2"
      target_is_set=1
      shift 2
      ;;
    --profile)
      profile="$2"
      shift 2
      ;;
    *)
      echo "unknown arg: $1"
      exit 1
      ;;
  esac
done

echo "==> building all packaged distributions"
if [[ "$target_is_set" -eq 1 ]]; then
  cargo run -p mgc-dist -- build-all --profile "$profile" --target "$target"
else
  cargo run -p mgc-dist -- build-all --profile "$profile"
fi
