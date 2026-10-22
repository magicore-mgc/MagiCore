# Temp Guard

Owns RAII cleanup for unique temporary files used by atomic blob writes.

**Ownership:** `TempGuard, parent_temp_path`. This module keeps the existing storage contract and delegates no registry operation to an external package manager.

**Tests:** existing registered integration coverage remains under `core/crates/mgc-registry-server/tests/`; owner-local tests are linked below when present.

---

# Temp Guard — Tiếng Việt

Sở hữu RAII cleanup cho file tạm duy nhất dùng trong ghi blob nguyên tử.

**Phạm vi sở hữu:** `TempGuard, parent_temp_path`. Module giữ nguyên contract storage hiện có và không chuyển thao tác registry sang package manager bên ngoài.

**Kiểm thử:** test integration đã đăng ký vẫn ở `core/crates/mgc-registry-server/tests/`; test gần owner được liên kết bên dưới nếu có.
