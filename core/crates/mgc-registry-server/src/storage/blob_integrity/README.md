# Blob Integrity

Streams SHA-256 verification and quarantines corrupted blob files for forensics.

**Ownership:** `hash_file_sha256, quarantine_corrupt_blob`. This module keeps the existing storage contract and delegates no registry operation to an external package manager.

**Tests:** existing registered integration coverage remains under `core/crates/mgc-registry-server/tests/`; owner-local tests are linked below when present.

---

# Blob Integrity — Tiếng Việt

Đọc tuần tự để xác minh SHA-256 và cách ly file blob hỏng phục vụ điều tra.

**Phạm vi sở hữu:** `hash_file_sha256, quarantine_corrupt_blob`. Module giữ nguyên contract storage hiện có và không chuyển thao tác registry sang package manager bên ngoài.

**Kiểm thử:** test integration đã đăng ký vẫn ở `core/crates/mgc-registry-server/tests/`; test gần owner được liên kết bên dưới nếu có.
