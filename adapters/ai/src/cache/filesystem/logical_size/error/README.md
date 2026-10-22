# Logical-size errors

The local error preserves which filesystem operation failed and chains the original `io::Error` through the shared `MgError` boundary.

Lỗi cục bộ giữ operation filesystem bị lỗi và nối `io::Error` gốc qua ranh giới `MgError` dùng chung.

Nearest source-chain cases are in `test/error.rs`.

Test source-chain gần nhất nằm ở `test/error.rs`.
