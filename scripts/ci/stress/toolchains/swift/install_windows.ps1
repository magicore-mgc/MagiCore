winget install --id Swift.Toolchain --exact --source winget --scope user --version 6.4.1 --accept-source-agreements --accept-package-agreements
if ($LASTEXITCODE -ne 0) { throw "WinGet failed to install Swift 6.4.1" }
$swiftRoot = Join-Path $env:LOCALAPPDATA "Programs\Swift\Toolchains"
$toolchain = Get-ChildItem -Path $swiftRoot -Directory | Where-Object { $_.Name -like "6.4.1*" } | Select-Object -First 1
if ($null -eq $toolchain) { throw "Swift 6.4.1 toolchain directory was not installed" }
$swiftBin = Join-Path $toolchain.FullName "usr\bin"
if (!(Test-Path (Join-Path $swiftBin "swift.exe"))) { throw "Swift executable is missing from the installed toolchain" }
$swiftRuntimeVersion = $toolchain.Name -replace '\+.*$', ''
$swiftRuntimeBin = Join-Path (Split-Path -Parent $swiftRoot) "Runtimes\$swiftRuntimeVersion\usr\bin"
if (!(Test-Path -LiteralPath (Join-Path $swiftRuntimeBin "swiftCore.dll") -PathType Leaf)) { throw "Swift runtime DLL is missing: $swiftRuntimeBin" }
$sdkRoot = [Environment]::GetEnvironmentVariable('SDKROOT', 'User')
if ([string]::IsNullOrWhiteSpace($sdkRoot) -or !(Test-Path -LiteralPath $sdkRoot -PathType Container)) { throw "Swift installer did not configure a usable User SDKROOT: $sdkRoot" }
Add-Content -Path $env:GITHUB_ENV -Value "SDKROOT=$sdkRoot" -Encoding utf8
Add-Content -Path $env:GITHUB_PATH -Value $swiftBin -Encoding utf8
Add-Content -Path $env:GITHUB_PATH -Value $swiftRuntimeBin -Encoding utf8
# Workflow exports apply to later steps; the current probe needs them now.
# Export workflow chỉ có hiệu lực step sau; probe hiện tại cần env trực tiếp.
$env:SDKROOT = $sdkRoot
$env:Path = "$swiftBin;$swiftRuntimeBin;$env:Path"
& (Join-Path $swiftBin "swift.exe") --version
