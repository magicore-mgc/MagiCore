# Bin Selection

Select and validate only package-declared executable names, preferring the package basename, then deterministic aliases, and requiring explicit choice for ambiguity.
Chỉ chọn/kiểm tên executable đã khai báo; ưu tiên basename package, sau đó alias xác định; package mơ hồ cần chọn tường minh.

## Files / File sở hữu

- `mod.rs` wires and re-exports this owner.
- `bin_selection.rs`: `bin_selection.rs` owns `valid_bin_name` and `select_bin_name`; `error.rs` owns the shared typed `InvalidBin`, `UndeclaredBin`, and `AmbiguousBin` diagnostics. `mod.rs` exports these APIs to package-runtime and managed-tool callers.
- `test/bin_selection.rs`: `test/bin_selection.rs` protects name validation and deterministic selection; `test/error.rs` checks downcast variants and exact English messages.
- `error.rs` and `test/error.rs`: typed failure and diagnostic contract where this owner emits a domain error.

`bin_selection.rs` owns `valid_bin_name` and `select_bin_name`; `error.rs` owns typed errors shared with managed tools. `mod.rs` exports the API.

## Test and limits / Test và giới hạn

`test/bin_selection.rs` protects name validation and deterministic selection; `test/error.rs` checks downcast variants and exact English messages. Uses declared metadata passed by `installed_metadata`; no filesystem mutation or network access. Ambiguous targets fail closed.

test/bin_selection.rs kiểm tra tên và quy tắc chọn; test/error.rs kiểm variant downcast và message tiếng Anh. Dùng metadata đã khai báo; không sửa file hay gọi mạng. Bin mơ hồ bị từ chối.

The registered unit command is `cargo test -p mgc --lib commands::package_runtime::`; it has not been run in this working session.
Lệnh unit đã đăng ký là `cargo test -p mgc --lib commands::package_runtime::`; chưa chạy trong phiên này.
