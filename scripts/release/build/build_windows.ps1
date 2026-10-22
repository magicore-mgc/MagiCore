# Windows native-target release build.
$ErrorActionPreference = "Stop"
$target = $env:MGC_RELEASE_TARGET
if ([string]::IsNullOrWhiteSpace($target)) {
  throw "MGC_RELEASE_TARGET is required"
}

cargo build --release --target $target --bin mgc --no-default-features --features all --locked
New-Item -ItemType Directory -Force -Path dist/magicore | Out-Null
Copy-Item "target/$target/release/mgc.exe" -Destination "dist/magicore/mgc.exe" -Force
cargo build --release --target $target --bin mgc --no-default-features --features web --locked
New-Item -ItemType Directory -Force -Path dist/magicore-web | Out-Null
Copy-Item "target/$target/release/mgc.exe" -Destination "dist/magicore-web/mgc.exe" -Force
