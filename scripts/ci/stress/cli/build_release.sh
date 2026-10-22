set -euo pipefail
if [[ "${RUNNER_OS}" == "macOS" ]]; then
  # Swift probes must not replace the Apple C compiler used by Go's C archive.
  # Probe Swift không được đổi compiler C dùng để build thư viện Go của esbuild.
  export CC="$(xcrun --toolchain default --find clang)"
  export CXX="$(xcrun --toolchain default --find clang++)"
  export SDKROOT="$(xcrun --sdk macosx --show-sdk-path)"
  echo "CC=${CC}" >> "${GITHUB_ENV}"
  echo "CXX=${CXX}" >> "${GITHUB_ENV}"
  echo "SDKROOT=${SDKROOT}" >> "${GITHUB_ENV}"
fi
cargo build --release --locked --bin mgc

