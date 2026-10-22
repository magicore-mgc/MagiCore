# Native Install

Build and invoke the current MagiCore binary with native web install policy, explicit lifecycle-script mode, inherited configured environment, and Install execution scope.
Tạo và gọi binary MagiCore hiện tại với policy cài web native, mode lifecycle script tường minh, env cấu hình kế thừa và Install scope.

## Files / File sở hữu

- `mod.rs` wires and re-exports this owner.
- `native_install.rs`: `native_install.rs` owns the invocation record, argv/options construction, warnings, and call to `run_project_binary_inherited`; `mod.rs` exports the installer operation.
- `test/native_install.rs`: `test/native_install.rs` checks current-MGC invocation options and script opt-in.

`native_install.rs giữ invocation, argv/options, warning và run_project_binary_inherited; mod.rs export operation cài.`

Errors outside this owner are propagated unchanged; this owner does not invent a local diagnostic variant.
Lỗi không thuộc owner này được propagate nguyên trạng; owner không tạo variant diagnostic giả.

## Test and limits / Test và giới hạn

`test/native_install.rs` checks current-MGC invocation options and script opt-in. No npm/pnpm/Bun/Yarn installer subprocess is launched. Underlying child errors are not converted to success.

test/native_install.rs kiểm options gọi MGC và script opt-in. Không chạy npm/pnpm/Bun/Yarn để cài. Lỗi child không bị đổi thành success.

The registered unit command is `cargo test -p mgc --lib commands::package_runtime::`; it has not been run in this working session.
Lệnh unit đã đăng ký là `cargo test -p mgc --lib commands::package_runtime::`; chưa chạy trong phiên này.
