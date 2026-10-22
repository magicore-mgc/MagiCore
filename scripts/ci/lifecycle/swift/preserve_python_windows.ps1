$pythonPath = $env:MGC_SWIFT_PYTHON_PATH
if ([string]::IsNullOrWhiteSpace($pythonPath) -or !(Test-Path -LiteralPath $pythonPath -PathType Leaf)) { throw "setup-python did not return an existing Python executable" }
$version = & $pythonPath --version
if ($version -notmatch '^Python 3\.10\.') { throw "Swift requires Python 3.10.x; found $version" }
$sourceBin = Split-Path -Parent $pythonPath
$pythonBin = Join-Path $env:RUNNER_TEMP "mgc-swift-python-3.10"
# Keep Swift's Python independent from installers that may mutate setup-python's toolcache.
# Tách Python của Swift khỏi toolcache setup-python mà các installer sau có thể thay đổi.
$PSNativeCommandUseErrorActionPreference = $false
& robocopy.exe $sourceBin $pythonBin /E /COPY:DAT /R:2 /W:1 /NFL /NDL /NJH /NJS
if ($LASTEXITCODE -gt 7) { throw "Failed to preserve Python 3.10 runtime (robocopy exit $LASTEXITCODE)" }
$preservedPython = Join-Path $pythonBin "python.exe"
if (!(Test-Path -LiteralPath $preservedPython -PathType Leaf)) { throw "Preserved Python 3.10 executable is missing" }
$preservedVersion = & $preservedPython --version
if ($preservedVersion -notmatch '^Python 3\.10\.') { throw "Preserved Swift Python must be 3.10.x; found $preservedVersion" }
Add-Content -Path $env:GITHUB_ENV -Value "MGC_SWIFT_PYTHON_BIN=$pythonBin" -Encoding utf8
