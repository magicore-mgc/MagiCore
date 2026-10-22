#!/usr/bin/env bash
# Install the release archive into an isolated user prefix.
# Cài archive phát hành vào prefix người dùng được cô lập.
set -euo pipefail
mkdir -p "$HOME/.magicore-test/bin"
if [[ "$RUNNER_OS" == "Windows" ]]; then
  # Flat zip: extract yields mgc.exe directly (no parent dir).
  # PowerShell handles native Windows paths reliably; bash
  # 'tar' on windows runners resolves to GNU tar which cannot
  # read zip (bsdtar lives at /c/Windows/System32/tar.exe but
  # Git's GNU tar shadows it in PATH).
  # Zip phẳng: giải nén ra thẳng mgc.exe (không thư mục cha).
  # PowerShell xử lý path Windows native chắc chắn; 'tar' của
  # bash trên windows runner trỏ GNU tar không đọc zip được
  # (bsdtar ở /c/Windows/System32/tar.exe nhưng GNU tar của
  # Git che mất trong PATH).
  powershell.exe -NoProfile -Command "Expand-Archive -LiteralPath (Join-Path (Get-Location) 'magicore.zip') -DestinationPath (Join-Path ([Environment]::GetFolderPath('UserProfile')) '.magicore-test') -Force"
  cp "$HOME/.magicore-test/mgc.exe" "$HOME/.magicore-test/bin/mgc.exe"
  chmod +x "$HOME/.magicore-test/bin/mgc.exe" || true
  "$HOME/.magicore-test/bin/mgc.exe" --version
else
  tar -xzf magicore.tar.gz -C "$HOME/.magicore-test"
  cp "$HOME/.magicore-test/$STAGE_DIR/mgc" "$HOME/.magicore-test/bin/mgc"
  chmod +x "$HOME/.magicore-test/bin/mgc"
  "$HOME/.magicore-test/bin/mgc" --version
fi
MGCBIN_PATH="$(command -v mgc || true)"
echo "mgc on PATH before export? -> ${MGCBIN_PATH:-none}"
