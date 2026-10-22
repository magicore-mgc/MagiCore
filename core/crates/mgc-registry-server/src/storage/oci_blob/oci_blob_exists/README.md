# OCI Blob Exists

Owns fail-closed OCI blob presence lookup: it validates the repository, reads the database path and reports true only when the backing file exists. Full digest verification remains on `get_oci_blob`.

**Error boundary:** validation and storage failures use the [shared OCI error owner](../error/README.md); the public method retains its existing `anyhow::Result<bool>` contract.

**Tests:** focused owner test: [`test/oci_blob_exists.rs`](test/oci_blob_exists.rs). `storage_hardening` also checks that quarantined bytes no longer count as present.

---

# OCI Blob Exists — Tiếng Việt

Sở hữu lookup tồn tại OCI blob theo fail-closed: validate repository, đọc path trong database và chỉ trả true khi file nền có thật. Kiểm tra digest đầy đủ vẫn do `get_oci_blob` thực hiện.

**Ranh giới lỗi:** lỗi validation và storage dùng [owner lỗi OCI dùng chung](../error/README.md); method công khai giữ hợp đồng `anyhow::Result<bool>` hiện có.

**Kiểm thử:** test tập trung: [`test/oci_blob_exists.rs`](test/oci_blob_exists.rs). `storage_hardening` cũng kiểm tra blob đã cách ly không còn được tính là tồn tại.
