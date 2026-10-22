# Oci Manifest

Owns OCI manifest storage, raw manifest lookup, and deletion by tag or digest.

**Ownership:** `put_oci_manifest, get_oci_manifest, get_oci_manifest_raw, delete_oci_manifest`. This module keeps the existing storage contract and delegates no registry operation to an external package manager.

**Tests:** existing registered integration coverage remains under `core/crates/mgc-registry-server/tests/`; owner-local tests are linked below when present.

---

# Oci Manifest — Tiếng Việt

Sở hữu lưu OCI manifest, truy vấn manifest thô và xóa theo tag hoặc digest.

**Phạm vi sở hữu:** `put_oci_manifest, get_oci_manifest, get_oci_manifest_raw, delete_oci_manifest`. Module giữ nguyên contract storage hiện có và không chuyển thao tác registry sang package manager bên ngoài.

**Kiểm thử:** test integration đã đăng ký vẫn ở `core/crates/mgc-registry-server/tests/`; test gần owner được liên kết bên dưới nếu có.
