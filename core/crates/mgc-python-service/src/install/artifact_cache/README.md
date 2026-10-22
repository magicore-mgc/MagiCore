# Python artifact cache

`artifact_cache.rs` checks the cache entry with `symlink_metadata`, opens it without following the final link/reparse point, checks the opened handle is a regular file, reads the bytes and compares SHA-256 before returning them. A missing entry remains a cache miss. The owner-local error preserves the existing outward split: inspection/read failures become `MgError::Other`; link/type/open/digest failures become `MgError::Integrity`.

## Cache artifact Python

`artifact_cache.rs` kiểm tra entry bằng `symlink_metadata`, mở không theo link/reparse point cuối, xác nhận handle là file thường, đọc bytes và so SHA-256 trước khi trả về. Entry vắng vẫn là cache miss. Lỗi local giữ nguyên phân loại public: lỗi inspect/read thành `MgError::Other`; lỗi link/type/open/digest thành `MgError::Integrity`.
