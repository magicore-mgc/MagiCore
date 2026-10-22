# Package both Windows CLI variants and write their SHA-256 sidecars.
$ErrorActionPreference = "Stop"
$artifact = $env:MGC_RELEASE_ARTIFACT_NAME
$webArtifact = $env:MGC_RELEASE_WEB_ARTIFACT_NAME
if ([string]::IsNullOrWhiteSpace($artifact) -or [string]::IsNullOrWhiteSpace($webArtifact)) {
  throw "release artifact names are required"
}

Compress-Archive -Path "dist/magicore/mgc.exe" -DestinationPath "$artifact" -Force
$hash = (Get-FileHash -Path "$artifact" -Algorithm SHA256).Hash.ToLower()
"$hash  $artifact" | Out-File -FilePath "${artifact}.sha256" -Encoding ascii
Compress-Archive -Path "dist/magicore-web/mgc.exe" -DestinationPath "$webArtifact" -Force
$webHash = (Get-FileHash -Path "$webArtifact" -Algorithm SHA256).Hash.ToLower()
"$webHash  $webArtifact" | Out-File -FilePath "${webArtifact}.sha256" -Encoding ascii
