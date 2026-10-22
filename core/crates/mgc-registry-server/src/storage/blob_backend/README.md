# Blob Backend

Implements local-filesystem and S3-compatible blob read/write operations.

**Ownership:** `BlobBackend, BlobBackend methods`. This module keeps the existing storage contract and delegates no registry operation to an external package manager.

**Tests:** existing registered integration coverage remains under `core/crates/mgc-registry-server/tests/`; owner-local tests are linked below when present.

---

# Blob Backend — Tiếng Việt

Triển khai đọc/ghi blob trên filesystem local và S3-compatible.

**Phạm vi sở hữu:** `BlobBackend, BlobBackend methods`. Module giữ nguyên contract storage hiện có và không chuyển thao tác registry sang package manager bên ngoài.

**Kiểm thử:** test integration đã đăng ký vẫn ở `core/crates/mgc-registry-server/tests/`; test gần owner được liên kết bên dưới nếu có.
