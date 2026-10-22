# Release operation owner. / Owner thao tác phát hành.
$ErrorActionPreference = "Stop"
New-Item -ItemType Directory -Force -Path lifecycle-test/web-project
Set-Location lifecycle-test/web-project

'{"name":"test-web","version":"1.0.0","dependencies":{"express":"^4.18.0"}}' | Out-File -FilePath package.json -Encoding utf8

& $env:MGC_RELEASE_BINARY --core web install

if (-not (Test-Path "node_modules/express")) {
  Write-Error "express not installed"
  exit 1
}

Write-Host "✅ Web install smoke test passed"
Write-Host "⚠️  Full lifecycle not verified in CI"
