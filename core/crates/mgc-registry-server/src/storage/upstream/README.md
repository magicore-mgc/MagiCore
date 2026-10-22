# Upstream

Owns the HTTP client and bounded registry-upstream response retrieval methods.

**Ownership:** `Upstream, fetch_json, fetch_bytes`. This module keeps the existing storage contract and delegates no registry operation to an external package manager.

**Tests:** existing registered integration coverage remains under `core/crates/mgc-registry-server/tests/`; owner-local tests are linked below when present.

---

# Upstream — Tiếng Việt

Sở hữu HTTP client và phương thức lấy response từ registry upstream có giới hạn.

**Phạm vi sở hữu:** `Upstream, fetch_json, fetch_bytes`. Module giữ nguyên contract storage hiện có và không chuyển thao tác registry sang package manager bên ngoài.

**Kiểm thử:** test integration đã đăng ký vẫn ở `core/crates/mgc-registry-server/tests/`; test gần owner được liên kết bên dưới nếu có.
