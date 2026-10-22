$archiveName = 'kotlin-compiler-2.4.21.zip'
$archivePath = Join-Path $env:RUNNER_TEMP $archiveName
$installDir = Join-Path $env:RUNNER_TEMP 'kotlin-compiler'
$expectedSha256 = '7cc140e76daf416a0424a5557f99afd7ca89ebd463ea1101261e3e01c33e75cd'
Invoke-WebRequest -Uri "https://github.com/JetBrains/kotlin/releases/download/v2.4.21/$archiveName" -OutFile $archivePath
$actualSha256 = (Get-FileHash -Path $archivePath -Algorithm SHA256).Hash.ToLowerInvariant()
if ($actualSha256 -ne $expectedSha256) { throw "Kotlin compiler archive SHA-256 mismatch: $actualSha256" }
Expand-Archive -Path $archivePath -DestinationPath $installDir -Force
$compilerBin = Join-Path $installDir 'kotlinc\bin'
$compilerLauncher = Join-Path $compilerBin 'kotlinc.bat'
if (!(Test-Path -LiteralPath $compilerLauncher -PathType Leaf)) { throw 'Kotlin compiler launcher is missing from the verified archive' }
Add-Content -Path $env:GITHUB_PATH -Value $compilerBin -Encoding utf8
& $compilerLauncher -version

