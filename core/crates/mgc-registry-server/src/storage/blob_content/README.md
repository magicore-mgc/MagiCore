# Blob Content

Owns package blob reads/writes, digest verification, and upstream tarball fetch.

**Owned API:** `compute_wire_digest, put_blob, get_blob, fetch_upstream_tarball`. Existing call paths and database behavior stay unchanged. Registered crate integration tests remain in `tests/` and are described as such.

# Blob Content — Tiếng Việt

Sở hữu đọc/ghi package blob, xác minh digest và lấy tarball upstream.

**API sở hữu:** `compute_wire_digest, put_blob, get_blob, fetch_upstream_tarball`. Đường gọi và hành vi database hiện tại được giữ nguyên. Test integration của crate tiếp tục ở `tests/`.
