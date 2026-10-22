# Mount OCI Blob

Owns cross-repository blob lookup, path construction, copy-on-missing and destination database upsert. The source size lookup is private to this owner. The public method signature and existing best-effort/copy behavior remain unchanged.

**Security boundary:** both repository segments are validated before filesystem paths are built; digest path decoding remains centralized. This operation currently copies source bytes without rehashing them; read-serving integrity verification remains in `get_oci_blob`.

**Error boundary:** internal failures are classified by the [shared OCI error owner](../error/README.md) and converted back to the existing `anyhow::Result<bool>` contract.

**Tests:** focused owner test: [`test/mount_oci_blob.rs`](test/mount_oci_blob.rs). HTTP authorization/mount behavior remains covered in `tests/registry/access_policy.rs`.

---

# Mount OCI Blob — Tiếng Việt

Sở hữu lookup blob giữa repository, dựng path, copy khi đích chưa có và upsert database row đích. Tra cứu kích thước nguồn chỉ dùng riêng trong owner này. Giữ nguyên chữ ký method và hành vi copy/best-effort hiện có.

**Ranh giới bảo mật:** validate cả hai segment repository trước khi tạo path filesystem; giải mã digest path vẫn dùng chung. Thao tác hiện tại copy bytes nguồn mà không rehash; xác minh integrity lúc phục vụ vẫn do `get_oci_blob` đảm nhiệm.

**Ranh giới lỗi:** lỗi nội bộ được phân loại tại [owner lỗi OCI dùng chung](../error/README.md), sau đó chuyển về hợp đồng `anyhow::Result<bool>` hiện có.

**Kiểm thử:** test tập trung: [`test/mount_oci_blob.rs`](test/mount_oci_blob.rs). Hành vi HTTP authorization/mount vẫn được kiểm tra tại `tests/registry/access_policy.rs`.
