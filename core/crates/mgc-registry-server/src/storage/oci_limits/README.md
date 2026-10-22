# Oci Limits

Centralizes OCI blob and manifest verification byte limits and boundary validation.

**Ownership:** `max_oci_blob_bytes, max_oci_manifest_verify_bytes, validate_oci_blob_size`. This module keeps the existing storage contract and delegates no registry operation to an external package manager.

**Tests:** existing registered integration coverage remains under `core/crates/mgc-registry-server/tests/`; owner-local tests are linked below when present.

---

# Oci Limits — Tiếng Việt

Tập trung giới hạn byte OCI blob/manifest và kiểm tra ranh giới.

**Phạm vi sở hữu:** `max_oci_blob_bytes, max_oci_manifest_verify_bytes, validate_oci_blob_size`. Module giữ nguyên contract storage hiện có và không chuyển thao tác registry sang package manager bên ngoài.

**Kiểm thử:** test integration đã đăng ký vẫn ở `core/crates/mgc-registry-server/tests/`; test gần owner được liên kết bên dưới nếu có.
