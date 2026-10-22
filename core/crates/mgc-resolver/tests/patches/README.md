# Resolver patch integration tests

`apply_patch.rs` covers successful application and context mismatch. `staged_validation.rs` checks validation before writes and malformed ranges/lines. `path_safety.rs` checks parent and symlink escapes. `tests/patches.rs` is only the target facade and public-route smoke check.

`apply_patch.rs` kiểm tra áp dụng và sai lệch context. `staged_validation.rs` kiểm tra xác thực trước khi ghi và dữ liệu lỗi. `path_safety.rs` kiểm tra thoát đường dẫn qua parent/symlink. `tests/patches.rs` chỉ nối target và smoke test route công khai.
