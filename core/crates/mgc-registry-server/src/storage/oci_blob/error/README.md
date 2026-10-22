# OCI Blob Error Owner

This owner defines operation-local error categories shared by OCI blob storage: preserved source errors, invalid repository/digest input, digest mismatch, configured size rejection, integrity failure, database failure and filesystem failure. The public `RegistryStore` methods still return `anyhow::Result`; `into_anyhow` preserves original `anyhow`, SQLx and I/O causes and reproduces the existing domain display strings.

This owner does not choose or perform retries. Retry safety belongs to the caller because it depends on the operation's side effects and request context.

**Tests:** [`test/error.rs`](test/error.rs) checks legacy messages, source preservation and retry classification. These tests have not been run in the source-first phase.

---

# Owner lỗi OCI Blob — Tiếng Việt

Owner này định nghĩa các nhóm lỗi dùng chung trong storage OCI blob: lỗi nguồn được giữ nguyên, repository/digest không hợp lệ, digest lệch, vượt giới hạn kích thước, lỗi integrity, database và filesystem. Các method `RegistryStore` vẫn trả `anyhow::Result`; `into_anyhow` giữ nguyên source `anyhow`, SQLx, I/O và tái tạo display string domain cũ.

Owner này không quyết định hoặc tự thực hiện retry. Caller phải tự đánh giá retry có an toàn không dựa trên side effect của operation và ngữ cảnh request.

**Kiểm thử:** [`test/error.rs`](test/error.rs) kiểm tra message cũ, giữ source và phân loại retry. Các test này chưa chạy trong phase source-first.
