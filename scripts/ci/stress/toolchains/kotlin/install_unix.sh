set -euo pipefail
archive_name="kotlin-compiler-2.4.21.zip"
expected_sha256="7cc140e76daf416a0424a5557f99afd7ca89ebd463ea1101261e3e01c33e75cd"
archive_path="${RUNNER_TEMP}/${archive_name}"
install_dir="${RUNNER_TEMP}/kotlin-compiler"
mkdir -p "${install_dir}"
curl --fail --location --retry 3 --silent --show-error \
  "https://github.com/JetBrains/kotlin/releases/download/v2.4.21/${archive_name}" \
  --output "${archive_path}"
python - "${archive_path}" "${expected_sha256}" <<'PY'
import hashlib
import pathlib
import sys

archive = pathlib.Path(sys.argv[1])
expected = sys.argv[2]
actual = hashlib.sha256(archive.read_bytes()).hexdigest()
if actual != expected:
    raise SystemExit(f"Kotlin compiler archive SHA-256 mismatch: {actual}")
PY
unzip -q "${archive_path}" -d "${install_dir}"
compiler_bin="${install_dir}/kotlinc/bin"
if [[ ! -x "${compiler_bin}/kotlinc" ]]; then
  echo "Kotlin compiler launcher is missing from the verified archive" >&2
  exit 1
fi
echo "${compiler_bin}" >> "${GITHUB_PATH}"
"${compiler_bin}/kotlinc" -version

