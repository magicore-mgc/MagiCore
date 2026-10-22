# Get OCI Blob

Owns the database lookup, file read, digest recheck and corruption quarantine on the OCI serving path. The public `RegistryStore::get_oci_blob` signature and `anyhow::Result` contract remain unchanged.

**Security boundary:** validate the repository before lookup; never return bytes until the stored content hashes to the requested digest; quarantine a mismatched blob before returning an error.

**Error boundary:** internal failures are classified by the [shared OCI error owner](../error/README.md) and converted back to the established `anyhow::Result` method contract.

**Tests:** focused owner test: [`test/get_oci_blob.rs`](test/get_oci_blob.rs). The registered `storage_hardening` integration test still verifies corruption rejection and quarantine.

---

# Get OCI Blob — Tiếng Việt

Sở hữu tra database, đọc file, kiểm tra lại digest và cách ly dữ liệu hỏng trên luồng phục vụ OCI. Giữ nguyên chữ ký `RegistryStore::get_oci_blob` và hợp đồng `anyhow::Result`.

**Ranh giới bảo mật:** validate repository trước khi lookup; không trả bytes trước khi nội dung lưu trữ có digest khớp yêu cầu; cách ly blob lệch digest rồi trả lỗi.

**Ranh giới lỗi:** lỗi nội bộ được phân loại tại [owner lỗi OCI dùng chung](../error/README.md), sau đó chuyển về hợp đồng method `anyhow::Result` hiện có.

**Kiểm thử:** test tập trung: [`test/get_oci_blob.rs`](test/get_oci_blob.rs). Integration test `storage_hardening` vẫn kiểm tra từ chối và cách ly blob hỏng.
