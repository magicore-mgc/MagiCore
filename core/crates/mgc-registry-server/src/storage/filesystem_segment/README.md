# Filesystem Segment

Rejects unsafe filesystem path segments used by OCI storage.

**Ownership:** `validate_fs_segment`. This module keeps the existing storage contract and delegates no registry operation to an external package manager.

**Tests:** existing registered integration coverage remains under `core/crates/mgc-registry-server/tests/`; owner-local tests are linked below when present.

---

# Filesystem Segment — Tiếng Việt

Từ chối segment filesystem không an toàn được dùng bởi OCI storage.

**Phạm vi sở hữu:** `validate_fs_segment`. Module giữ nguyên contract storage hiện có và không chuyển thao tác registry sang package manager bên ngoài.

**Kiểm thử:** test integration đã đăng ký vẫn ở `core/crates/mgc-registry-server/tests/`; test gần owner được liên kết bên dưới nếu có.
