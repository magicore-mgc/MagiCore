set -euo pipefail
case "$(uname -s):$(uname -m)" in
  Linux:x86_64)
    archive_platform="linux_amd64"
    expected_sha256="38034b38d03096399a7a1264a14561a8029b2b496fd1f169452656e53968f1b7"
    ;;
  Darwin:arm64)
    archive_platform="darwin_arm64"
    expected_sha256="85e1bc78ab45bdc5f82a7bac3929e37d072834a584d22583a2498dc67bb624cd"
    ;;
  Darwin:x86_64)
    archive_platform="darwin_amd64"
    expected_sha256="794a36c3f7ebf6c58f2089c226068a539d1705ba358a5679b51671c8dc7ebc7b"
    ;;
  *)
    echo "No pinned Terraform archive exists for this runner architecture" >&2
    exit 1
    ;;
esac
archive_name="terraform_1.16.6_${archive_platform}.zip"
archive_path="${RUNNER_TEMP}/${archive_name}"
install_dir="${RUNNER_TEMP}/terraform-cli"
mkdir -p "${install_dir}"
curl --fail --location --retry 3 --silent --show-error \
  "https://releases.hashicorp.com/terraform/1.16.6/${archive_name}" \
  --output "${archive_path}"
python - "${archive_path}" "${expected_sha256}" <<'PY'
import hashlib
import pathlib
import sys

archive = pathlib.Path(sys.argv[1])
expected = sys.argv[2]
actual = hashlib.sha256(archive.read_bytes()).hexdigest()
if actual != expected:
    raise SystemExit(f"Terraform archive SHA-256 mismatch: {actual}")
PY
unzip -q "${archive_path}" -d "${install_dir}"
echo "${install_dir}" >> "${GITHUB_PATH}"
"${install_dir}/terraform" version

