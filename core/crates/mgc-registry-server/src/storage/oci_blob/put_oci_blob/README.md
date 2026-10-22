# Put OCI Blob

Owns repository validation, digest/size checks, atomic blob publication and the matching database upsert. The public `RegistryStore::put_oci_blob` signature and `anyhow::Result` contract remain unchanged.

**Security boundary:** validate the repository segment before constructing a path; verify the declared digest against bytes; enforce the configured size; sync the unique temporary file before rename; write the database row only after publication.

**Error boundary:** internal failures are classified by the [shared OCI error owner](../error/README.md) and converted back to the established `anyhow::Result` method contract.

**Tests:** focused owner test: [`test/put_oci_blob.rs`](test/put_oci_blob.rs). Existing end-to-end coverage remains in `tests/storage_hardening.rs` and registry integration tests.

---

# Put OCI Blob — Tiếng Việt

Sở hữu kiểm tra repository, digest/kích thước, publish blob nguyên tử và upsert database tương ứng. Giữ nguyên chữ ký `RegistryStore::put_oci_blob` và hợp đồng `anyhow::Result`.

**Ranh giới bảo mật:** validate segment repository trước khi tạo path; xác minh digest đã khai báo với bytes; giới hạn kích thước theo cấu hình; sync file tạm duy nhất trước rename; chỉ ghi database sau khi publish thành công.

**Ranh giới lỗi:** lỗi nội bộ được phân loại tại [owner lỗi OCI dùng chung](../error/README.md), sau đó chuyển về hợp đồng method `anyhow::Result` hiện có.

**Kiểm thử:** test tập trung: [`test/put_oci_blob.rs`](test/put_oci_blob.rs). E2E hiện có vẫn nằm tại `tests/storage_hardening.rs` và các integration test registry.
