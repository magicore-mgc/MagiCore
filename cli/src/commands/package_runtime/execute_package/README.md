# Execute Package

Execute a previously verified binary with the established working directory, PATH ordering, environment policy, process scope, log path, and launcher lifetime.
Chạy binary đã xác minh với cwd, thứ tự PATH, env, process scope, log path và lifetime launcher hiện hành.

## Files / File sở hữu

- `mod.rs` wires and re-exports this owner.
- `execute_package.rs`: `execute_package.rs` owns `runner_path` and `execute_verified_binary`; `mod.rs` exports the execution entry. Child-process errors are propagated unchanged.
- `test/execute_package.rs`: `test/execute_package.rs` checks native and Node PATH ordering.

execute_package.rs owns `runner_path` and `execute_verified_binary`; `mod.rs` exports the execution entry. Lỗi process con được propagate nguyên trạng.

Errors outside this owner are propagated unchanged; this owner does not invent a local diagnostic variant.
Lỗi không thuộc owner này được propagate nguyên trạng; owner không tạo variant diagnostic giả.

## Test and limits / Test và giới hạn

`test/execute_package.rs` checks native and Node PATH ordering. Does not resolve, install, select, or verify package binaries; those steps belong to adjacent owners.

test/execute_package.rs kiểm tra thứ tự PATH cho native và Node. Không resolve, cài, chọn hay xác minh bin; các bước đó thuộc owner bên cạnh.

The registered unit command is `cargo test -p mgc --lib commands::package_runtime::`; it has not been run in this working session.
Lệnh unit đã đăng ký là `cargo test -p mgc --lib commands::package_runtime::`; chưa chạy trong phiên này.
