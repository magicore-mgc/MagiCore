# OCI Blob Storage

This folder wires five independently reviewable OCI blob operations and one shared error owner while keeping the existing `RegistryStore` methods and storage facade. Wire-digest calculation remains owned by `blob_content` and is shared within `storage`; descriptor verification remains in the adjacent `oci_blob_descriptor` owner. `error/` owns typed operation outcomes and maps them to the existing `anyhow::Result` boundary without changing method signatures, display text or HTTP status mapping.

| Owner | Responsibility | Focused test |
|---|---|---|
| `error/` | Typed domain, database and filesystem failures; compatibility conversion | `error/test/error.rs` |
| `put_oci_blob/` | Validate repo/digest/size, atomically publish bytes, then upsert the row | `put_oci_blob/test/put_oci_blob.rs` |
| `get_oci_blob/` | Read and rehash bytes; quarantine mismatch | `get_oci_blob/test/get_oci_blob.rs` |
| `oci_blob_exists/` | Require both DB row and backing file | `oci_blob_exists/test/oci_blob_exists.rs` |
| `delete_oci_blob/` | Best-effort file removal and row deletion | `delete_oci_blob/test/delete_oci_blob.rs` |
| `mount_oci_blob/` | Copy a blob across repositories and upsert destination metadata | `mount_oci_blob/test/mount_oci_blob.rs` |

**Existing acceptance coverage:** `tests/storage_hardening.rs`, `tests/registry/oci_publish.rs`, `tests/registry/access_policy.rs`, and handler-local tests remain at their registered locations. They have not been run in this source-first phase.

**Known behavior follow-ups:** delete still ignores file-removal errors before deleting its DB row. Mount still copies source bytes without rehashing; read-serving integrity verification is performed by `get_oci_blob`. This batch documents and preserves both behaviors; neither is represented as newly verified.

---

# OCI Blob Storage — Tiếng Việt

Folder này nối năm thao tác OCI blob có thể review riêng và một owner lỗi dùng chung, đồng thời giữ nguyên method `RegistryStore` và storage facade. Tính wire digest tiếp tục thuộc `blob_content` và được dùng chung bên trong `storage`; xác minh descriptor tiếp tục thuộc owner kế bên `oci_blob_descriptor`. `error/` sở hữu lỗi typed và chuyển về biên `anyhow::Result` hiện có, không đổi signature, display message hoặc HTTP status mapping.

| Owner | Trách nhiệm | Test tập trung |
|---|---|---|
| `error/` | Lỗi typed domain/database/filesystem; chuyển đổi giữ tương thích | `error/test/error.rs` |
| `put_oci_blob/` | Validate repo/digest/size, publish bytes nguyên tử rồi upsert row | `put_oci_blob/test/put_oci_blob.rs` |
| `get_oci_blob/` | Đọc và rehash bytes; cách ly khi lệch | `get_oci_blob/test/get_oci_blob.rs` |
| `oci_blob_exists/` | Yêu cầu có cả DB row và file nền | `oci_blob_exists/test/oci_blob_exists.rs` |
| `delete_oci_blob/` | Gỡ file best-effort và xóa row | `delete_oci_blob/test/delete_oci_blob.rs` |
| `mount_oci_blob/` | Copy blob qua repository và upsert metadata đích | `mount_oci_blob/test/mount_oci_blob.rs` |

**Coverage hiện có:** `tests/storage_hardening.rs`, `tests/registry/oci_publish.rs`, `tests/registry/access_policy.rs` và test gần handler vẫn ở vị trí đã đăng ký. Chưa chạy chúng trong phase source-first này.

**Hành vi cần xử lý riêng:** delete vẫn bỏ qua lỗi gỡ file trước khi xóa DB row. Mount vẫn copy bytes nguồn mà không rehash; xác minh integrity lúc phục vụ do `get_oci_blob` thực hiện. Batch này ghi nhận và giữ nguyên hai hành vi, không tuyên bố đã xác minh mới.
