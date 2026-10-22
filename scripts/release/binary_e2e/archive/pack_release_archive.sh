#!/usr/bin/env bash
# Pack the release-shaped archive using the detected host triple.
# Đóng gói archive theo cấu trúc phát hành bằng host triple đã nhận diện.
set -euo pipefail
VERSION=""
in_workspace_package=false
while IFS= read -r line; do
  if [[ "$line" == "[workspace.package]" ]]; then
    in_workspace_package=true
    continue
  fi
  if [[ "$in_workspace_package" == true && "$line" == "["* ]]; then
    break
  fi
  if [[ "$in_workspace_package" == true && "$line" =~ ^version[[:space:]]*=[[:space:]]*\"([^\"]+)\"$ ]]; then
    VERSION="${BASH_REMATCH[1]}"
    break
  fi
done < Cargo.toml
if [[ -z "$VERSION" ]]; then
  echo "workspace version is missing from Cargo.toml" >&2
  exit 1
fi
ARCH="$HOST_TRIPLE"
STAGE="magicore-$VERSION-$ARCH"
mkdir -p "stage/$STAGE"
if [[ "$RUNNER_OS" == "Windows" ]]; then
  cp target/release/mgc.exe "stage/$STAGE/"
else
  cp target/release/mgc "stage/$STAGE/"
fi
if [[ "$RUNNER_OS" == "Windows" ]]; then
  # Windows archive: zip with the binary at the archive root
  # (Compress-Archive keeps the entry as the bare file name —
  # no parent dir; verified flat layout, extracted flat).
  # Archive Windows: zip với binary ở root (Compress-Archive
  # giữ entry chỉ tên file — không thư mục cha; layout phẳng,
  # giải nén phẳng).
  cd "stage/$STAGE"
  powershell.exe -NoProfile -Command "Compress-Archive -Path mgc.exe -DestinationPath '../../magicore.zip' -Force"
  cd ../..
else
  tar -czf "magicore.tar.gz" -C stage "$STAGE"
fi
echo "STAGE_DIR=$STAGE" >> "$GITHUB_ENV"
