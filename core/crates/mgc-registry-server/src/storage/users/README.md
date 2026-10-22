# Users

Loads, creates, upserts, and revokes persistent registry users.

**Ownership:** `load_users, create_user, upsert_user, delete_user_by_name, delete_user_by_token`. This module keeps the existing storage contract and delegates no registry operation to an external package manager.

**Tests:** existing registered integration coverage remains under `core/crates/mgc-registry-server/tests/`; owner-local tests are linked below when present.

---

# Users — Tiếng Việt

Đọc, tạo, upsert và thu hồi user registry lưu bền.

**Phạm vi sở hữu:** `load_users, create_user, upsert_user, delete_user_by_name, delete_user_by_token`. Module giữ nguyên contract storage hiện có và không chuyển thao tác registry sang package manager bên ngoài.

**Kiểm thử:** test integration đã đăng ký vẫn ở `core/crates/mgc-registry-server/tests/`; test gần owner được liên kết bên dưới nếu có.
