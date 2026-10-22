set -euo pipefail
case "${RUNNER_OS}" in
  Linux)
    archive_name="Godot_v4.7.2-stable_linux.x86_64.zip"
    expected_sha512="9aa00f7a605200940bce3027a567b782f49bd8e940dd06ae9e987bd65aee1b1467edd56ed84fcdcbdd44354bf613bdbb4e5d2913e925850368e150c59ed54c65"
    ;;
  macOS)
    archive_name="Godot_v4.7.2-stable_macos.universal.zip"
    expected_sha512="38aa16e5bba2083941fc5b3e54be0089bd4cc35e32415f5b9fd9a8a6a7b9818255d44532ea8ef94b5aef56c4b407c2d634fa4f657e4ebe681ebbf59b7bac69ca"
    ;;
  *)
    echo "Unsupported runner OS for pinned Godot archive" >&2
    exit 1
    ;;
esac
archive_path="${RUNNER_TEMP}/${archive_name}"
extract_dir="${RUNNER_TEMP}/godot-extract"
install_dir="${RUNNER_TEMP}/godot-bin"
mkdir -p "${extract_dir}" "${install_dir}"
curl --fail --location --retry 3 --silent --show-error \
  "https://github.com/godotengine/godot-builds/releases/download/4.7.2-stable/${archive_name}" \
  --output "${archive_path}"
python - "${archive_path}" "${expected_sha512}" <<'PY'
import hashlib
import pathlib
import sys

archive = pathlib.Path(sys.argv[1])
expected = sys.argv[2]
actual = hashlib.sha512(archive.read_bytes()).hexdigest()
if actual != expected:
    raise SystemExit(f"Godot archive SHA-512 mismatch: {actual}")
PY
unzip -q "${archive_path}" -d "${extract_dir}"
if [[ "${RUNNER_OS}" == "macOS" ]]; then
  godot_source="$(find "${extract_dir}" -path '*/Godot.app/Contents/MacOS/Godot' -type f -print -quit)"
else
  godot_source="$(find "${extract_dir}" -maxdepth 2 -type f -name 'Godot_v4.7.2-stable_linux.x86_64' -print -quit)"
fi
if [[ -z "${godot_source}" ]]; then
  echo "Godot executable is missing from the verified archive" >&2
  exit 1
fi
chmod +x "${godot_source}"
ln -s "${godot_source}" "${install_dir}/godot"
echo "${install_dir}" >> "${GITHUB_PATH}"
"${install_dir}/godot" --headless --version
