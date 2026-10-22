# Delete OCI Blob

Owns the database lookup, best-effort backing-file removal and database-row deletion for an OCI blob. This extraction preserves the current behavior and public `anyhow::Result<bool>` signature; it does not redefine shared-file ownership or deletion recovery.

**Known follow-up:** filesystem removal errors are intentionally ignored by the existing implementation before the row is deleted. Changing that failure policy requires a separate behavior/security change and a regression decision.

**Error boundary:** database failures use the [shared OCI error owner](../error/README.md). The existing best-effort filesystem removal remains ignored by design in this extraction.

**Tests:** focused owner test: [`test/delete_oci_blob.rs`](test/delete_oci_blob.rs). HTTP DELETE coverage remains in `src/oci/blob_read` and registry integration tests.

---

# Delete OCI Blob — Tiếng Việt

Sở hữu tra database, gỡ file nền best-effort và xóa database row của OCI blob. Lần tách này giữ nguyên hành vi và chữ ký `anyhow::Result<bool>` hiện có; không tự định nghĩa lại quyền sở hữu file dùng chung hay cách phục hồi khi xóa lỗi.

**Việc cần theo dõi:** implementation hiện có cố ý bỏ qua lỗi gỡ file trước khi xóa row. Thay đổi chính sách lỗi này cần một thay đổi hành vi/bảo mật riêng và quyết định hồi quy tương ứng.

**Ranh giới lỗi:** lỗi database dùng [owner lỗi OCI dùng chung](../error/README.md). Lần tách này vẫn giữ hành vi bỏ qua lỗi gỡ filesystem như trước.

**Kiểm thử:** test tập trung: [`test/delete_oci_blob.rs`](test/delete_oci_blob.rs). Coverage HTTP DELETE vẫn nằm tại `src/oci/blob_read` và integration test registry.
