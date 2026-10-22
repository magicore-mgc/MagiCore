$archiveName = 'terraform_1.16.6_windows_amd64.zip'
$archivePath = Join-Path $env:RUNNER_TEMP $archiveName
$installDir = Join-Path $env:RUNNER_TEMP 'terraform-cli'
$expectedSha256 = '6ea2c186e0e76a113af30fbcdcf7a4b4c70756317994a04929e5c90b9e94b1df'
Invoke-WebRequest -Uri "https://releases.hashicorp.com/terraform/1.16.6/$archiveName" -OutFile $archivePath
$actualSha256 = (Get-FileHash -Path $archivePath -Algorithm SHA256).Hash.ToLowerInvariant()
if ($actualSha256 -ne $expectedSha256) { throw "Terraform archive SHA-256 mismatch: $actualSha256" }
Expand-Archive -Path $archivePath -DestinationPath $installDir -Force
Add-Content -Path $env:GITHUB_PATH -Value $installDir -Encoding utf8
& (Join-Path $installDir 'terraform.exe') version

