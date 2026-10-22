# Release operation owner. / Owner thao tác phát hành.
$ErrorActionPreference = "Stop"
cargo install cargo-cyclonedx --version 0.5.9 --locked

# Generate for all-features binary (matches build flags exactly)
Set-Location cli
cargo cyclonedx `
  --format json `
  --spec-version 1.4 `
  --no-default-features `
  --features all `
  --target $env:MGC_RELEASE_TARGET
Set-Location ..
Move-Item -Path cli/mgc.cdx.json -Destination "magicore-$env:MGC_RELEASE_VERSION-$env:MGC_RELEASE_OS_NAME-$env:MGC_RELEASE_ARCH-sbom.json" -Force

# Generate for web-only binary (matches build flags exactly)
Set-Location cli
cargo cyclonedx `
  --format json `
  --spec-version 1.4 `
  --no-default-features `
  --features web `
  --target $env:MGC_RELEASE_TARGET
Set-Location ..
Move-Item -Path cli/mgc.cdx.json -Destination "magicore-web-$env:MGC_RELEASE_VERSION-$env:MGC_RELEASE_OS_NAME-$env:MGC_RELEASE_ARCH-sbom.json" -Force

# Verify SBOMs
Get-ChildItem -Filter "*-sbom.json" | ForEach-Object {
  $sbomData = Get-Content $_.FullName | ConvertFrom-Json

  # Check required fields (P1 finding #6)
  if ($sbomData.bomFormat -ne "CycloneDX") {
    Write-Error "Invalid bomFormat: $($_.Name)"
    exit 1
  }
  if (-not $sbomData.specVersion) {
    Write-Error "Missing specVersion: $($_.Name)"
    exit 1
  }
  if (-not $sbomData.metadata.component) {
    Write-Error "Missing metadata.component: $($_.Name)"
    exit 1
  }
  if (-not $sbomData.components) {
    Write-Error "Missing components: $($_.Name)"
    exit 1
  }

  $componentCount = $sbomData.components.Count
  if ($componentCount -lt 10) {
    Write-Error "Too few components ($componentCount): $($_.Name)"
    exit 1
  }

  Write-Host "✅ Valid SBOM: $($_.Name) ($componentCount components)"
}
