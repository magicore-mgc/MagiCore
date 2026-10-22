$archiveName = 'Godot_v4.7.2-stable_win64.exe.zip'
$archivePath = Join-Path $env:RUNNER_TEMP $archiveName
$extractDir = Join-Path $env:RUNNER_TEMP 'godot-extract'
$installDir = Join-Path $env:RUNNER_TEMP 'godot-bin'
$expectedSha512 = '83decd58fdf67b9d657958a1ae6bf1929c20785315a81effe245874cdc57acb709bf868e00778a96984338c1b29dafdb453c6847747694621c6ecf5da2259993'
New-Item -ItemType Directory -Force -Path $extractDir, $installDir | Out-Null
Invoke-WebRequest -Uri "https://github.com/godotengine/godot-builds/releases/download/4.7.2-stable/$archiveName" -OutFile $archivePath
$actualSha512 = (Get-FileHash -Path $archivePath -Algorithm SHA512).Hash.ToLowerInvariant()
if ($actualSha512 -ne $expectedSha512) { throw "Godot archive SHA-512 mismatch: $actualSha512" }
Expand-Archive -Path $archivePath -DestinationPath $extractDir -Force
$godotSource = Get-ChildItem -Path $extractDir -Recurse -Filter 'Godot_v4.7.2-stable_win64.exe' | Select-Object -First 1
if ($null -eq $godotSource) { throw 'Godot executable is missing from the verified archive' }
$godotExe = Join-Path $installDir 'godot.exe'
Copy-Item -Path $godotSource.FullName -Destination $godotExe -Force
Add-Content -Path $env:GITHUB_PATH -Value $installDir -Encoding utf8
& $godotExe --headless --version

