$gccCommand = Get-Command gcc.exe -CommandType Application -ErrorAction SilentlyContinue
$gccCandidates = @('C:\mingw64\bin', 'C:\msys64\mingw64\bin', 'C:\msys64\ucrt64\bin')
if ($gccCommand) {
  $gccCandidates = @((Split-Path -Parent $gccCommand.Source)) + $gccCandidates
}
$gccBin = $gccCandidates | Where-Object {
  (Test-Path -LiteralPath (Join-Path $_ 'gcc.exe') -PathType Leaf) -and
  (Test-Path -LiteralPath (Join-Path $_ 'g++.exe') -PathType Leaf)
} | Select-Object -First 1
if (!$gccBin) { throw 'GCC and G++ are missing from the hosted runner' }
Add-Content -Path $env:GITHUB_PATH -Value $gccBin -Encoding utf8
& (Join-Path $gccBin 'gcc.exe') --version
& (Join-Path $gccBin 'g++.exe') --version
