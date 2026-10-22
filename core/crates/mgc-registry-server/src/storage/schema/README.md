# Schema

Creates and upgrades the registry metadata schema through the existing migration and initialization sequence.

**Ownership:** `init_schema`. This module keeps the existing storage contract and delegates no registry operation to an external package manager.

**Tests:** existing registered integration coverage remains under `core/crates/mgc-registry-server/tests/`; owner-local tests are linked below when present.

---

# Schema — Tiếng Việt

Tạo và khởi tạo schema metadata registry theo đúng chuỗi migration hiện có.

**Phạm vi sở hữu:** `init_schema`. Module giữ nguyên contract storage hiện có và không chuyển thao tác registry sang package manager bên ngoài.

**Kiểm thử:** test integration đã đăng ký vẫn ở `core/crates/mgc-registry-server/tests/`; test gần owner được liên kết bên dưới nếu có.
