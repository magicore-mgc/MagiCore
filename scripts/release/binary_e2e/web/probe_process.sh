#!/usr/bin/env bash
# Share the HTTP probe and cross-platform child-process shutdown policy.
# Dùng chung HTTP probe và chính sách dừng process con đa nền tảng.
port_responds() {
  curl --connect-timeout 1 --max-time 2 -sS -o /dev/null "http://127.0.0.1:4315/" 2>/dev/null \
    || curl --connect-timeout 1 --max-time 2 -sS -o /dev/null "http://localhost:4315/" 2>/dev/null
}
stop_probe_process() {
  local process_pid="$1"
  if [[ "$RUNNER_OS" == "Windows" ]]; then
    # Git-Bash $! may be an MSYS PID, not the Win32 PID owning
    # the listening socket. Kill the launcher tree, then the
    # exact process(es) still bound to this isolated probe port.
    taskkill //F //T //PID "$process_pid" > /dev/null 2>&1 || true
    powershell.exe -NoProfile -Command '
      $ErrorActionPreference = "SilentlyContinue"
      $owners = Get-NetTCPConnection -State Listen -LocalPort 4315 |
        Select-Object -ExpandProperty OwningProcess -Unique
      foreach ($ownerPid in $owners) {
        & taskkill.exe /F /T /PID $ownerPid | Out-Null
      }
    '
  else
    kill "$process_pid" 2>/dev/null || true
    pkill -f "vite.*4315" 2>/dev/null || true
  fi
}
