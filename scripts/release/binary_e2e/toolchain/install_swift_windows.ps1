# Install and verify the official signed Windows Swift toolchain.
# Nạp và xác minh toolchain Swift Windows chính thức có chữ ký.
$ErrorActionPreference = 'Stop'
$installerPath = Join-Path $env:RUNNER_TEMP "swift-6.4.1-RELEASE-windows10.exe"
Invoke-WebRequest -Uri "https://download.swift.org/swift-6.4.1-release/windows10/swift-6.4.1-RELEASE/swift-6.4.1-RELEASE-windows10.exe" -OutFile $installerPath
$expectedSha256 = "9382E14919EEAE5E7B5DBFE0575452FE2154178E089A595C8B44D03960D91D59"
$actualSha256 = (Get-FileHash -Algorithm SHA256 -Path $installerPath).Hash
if ($actualSha256 -ne $expectedSha256) { throw "Swift 6.4.1 installer SHA-256 mismatch: $actualSha256" }
$signature = Get-AuthenticodeSignature -FilePath $installerPath
if ($signature.Status -ne 'Valid') { throw "Swift installer Authenticode signature is not valid: $($signature.Status)" }
$install = Start-Process -FilePath $installerPath -ArgumentList @('/quiet', '/norestart') -Wait -PassThru
if ($install.ExitCode -ne 0) { throw "Swift installer failed with exit code $($install.ExitCode)" }
$swiftRoot = Join-Path $env:LOCALAPPDATA "Programs\Swift\Toolchains"
$toolchain = Get-ChildItem -Path $swiftRoot -Directory | Where-Object { $_.Name -like "6.4.1*" } | Select-Object -First 1
if ($null -eq $toolchain) { throw "Swift 6.4.1 toolchain directory was not installed" }
$swiftBin = Join-Path $toolchain.FullName "usr\bin"
if (!(Test-Path (Join-Path $swiftBin "swift.exe"))) { throw "Swift executable is missing from the installed toolchain" }
$swiftRuntimeVersion = $toolchain.Name -replace '\+.*$', ''
$swiftRuntimeBin = Join-Path (Split-Path -Parent $swiftRoot) "Runtimes\$swiftRuntimeVersion\usr\bin"
if (!(Test-Path -LiteralPath (Join-Path $swiftRuntimeBin "swiftCore.dll") -PathType Leaf)) { throw "Swift runtime DLL is missing: $swiftRuntimeBin" }
# The installer writes SDKROOT at User scope; copy it into later workflow steps.
# Installer ghi SDKROOT ở cấp User; chuyển nó sang các bước workflow tiếp theo.
$sdkRoot = [Environment]::GetEnvironmentVariable('SDKROOT', 'User')
if ([string]::IsNullOrWhiteSpace($sdkRoot) -or !(Test-Path -LiteralPath $sdkRoot -PathType Container)) { throw "Swift installer did not configure a usable User SDKROOT: $sdkRoot" }
Add-Content -Path $env:GITHUB_ENV -Value "SDKROOT=$sdkRoot" -Encoding utf8
Write-Host "Swift SDKROOT: $sdkRoot"
Add-Content -Path $env:GITHUB_PATH -Value $swiftBin -Encoding utf8
# Swift packages load DLLs from the separate runtime installation.
# Gói Swift nạp DLL từ thư mục runtime cài riêng.
Add-Content -Path $env:GITHUB_PATH -Value $swiftRuntimeBin -Encoding utf8
Write-Host "Swift runtime directory: $swiftRuntimeBin"
& (Join-Path $swiftBin "swift.exe") --version
