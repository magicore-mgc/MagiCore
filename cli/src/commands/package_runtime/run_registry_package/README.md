# Run Registry Package

Preserve the ordered package-run use case and reject caller execution directories that are symlinks or non-directories before installation.
Giữ use case chạy package theo đúng thứ tự và từ chối cwd caller là symlink/không phải thư mục trước khi cài.

## Files / File sở hữu

- `mod.rs` wires and re-exports this owner.
- `run_registry_package.rs`: `run_registry_package.rs` owns `run_registry_package_in`, `RegistryPackageReport`, and the cwd gate; `error.rs` owns its typed cwd failure. `mod.rs` exports the stable use case.
- `test/run_registry_package.rs`: `test/run_registry_package.rs` checks real directory, regular-file and symlink cases; `test/error.rs` checks typed output.
- `error.rs` and `test/error.rs`: typed failure and diagnostic contract where this owner emits a domain error.

`run_registry_package.rs sở hữu use case, report và cwd gate; error.rs giữ lỗi cwd typed. mod.rs export use case ổn định.`

## Test and limits / Test và giới hạn

`test/run_registry_package.rs` checks real directory, regular-file and symlink cases; `test/error.rs` checks typed output. The owner coordinates adjacent steps but does not implement registry transport, installation or process execution.

test/run_registry_package.rs kiểm thư mục thật/file/symlink; test/error.rs kiểm output typed. Owner điều phối; không tự làm registry transport, cài hay chạy process.

The registered unit command is `cargo test -p mgc --lib commands::package_runtime::`; it has not been run in this working session.
Lệnh unit đã đăng ký là `cargo test -p mgc --lib commands::package_runtime::`; chưa chạy trong phiên này.
