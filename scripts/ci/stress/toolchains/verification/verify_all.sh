set -euo pipefail
runtime_tools=(bash node bun deno python go dart rustc cargo swift java dotnet php kotlinc terraform godot zig)
case "${RUNNER_OS}" in
  macOS)
    runtime_tools+=(clang clang++)
    ;;
  Linux|Windows)
    runtime_tools+=(gcc g++)
    ;;
  *)
    echo "Unsupported runner OS for language probes: ${RUNNER_OS}" >&2
    exit 1
    ;;
esac
for executable in "${runtime_tools[@]}"; do
  if ! command -v "${executable}" >/dev/null 2>&1; then
    echo "Required language probe tool is missing from PATH: ${executable}" >&2
    exit 1
  fi
done
test "$(node --version)" = "v26.12.0"
deno --version | head -1 | grep -F "deno 2.9.7"
rustc --version | grep -F "rustc 1.99.0"
go version | grep -F "go1.27.2"
python --version 2>&1 | grep -F "Python 3.15.0" || python3 --version 2>&1 | grep -F "Python 3.15.0"
java_version="$(java -version 2>&1)"
printf '%s\n' "$java_version"
grep -F 'build 27+35' <<< "$java_version"
dotnet --version | grep -F "10.0.402"
php --version | head -1 | grep -F "8.5.11"
flutter --version | head -1 | grep -F "3.47.7"
swift --version | head -1 | grep -F "6.4.1"
kotlinc -version 2>&1 | grep -F "2.4.21"
terraform version | head -1 | grep -F "1.16.6"
godot --headless --version | grep -F "4.7.2"
zig version | grep -F "0.17.0"
