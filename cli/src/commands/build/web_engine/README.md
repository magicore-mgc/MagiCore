# Web build engine / Engine build Web

## Responsibility / Trách nhiệm

This folder owns Web build orchestration and focused helpers for target selection, framework script routing, Node local executables, entry discovery, native engine builds, allowlisted execution, runtime selection and Rust build. `mod.rs` only exports stable internal APIs.

Folder này sở hữu điều phối build Web cùng helper riêng cho target, route script framework, executable Node cục bộ, tìm entry, build native engine, process allowlist, chọn runtime và build Rust. `mod.rs` chỉ export API nội bộ ổn định.

## Error and test boundaries / Ranh giới lỗi và test

Script-policy, Node-path, entrypoint, native-artifact and managed-source failures have typed local errors with stable English diagnostics. Shared filesystem/compiler/executor errors keep their source chain. Existing tests remain under `cli/src/test/build_test.rs`; no tests were run in S162.

Lỗi policy script, Node path, entrypoint, native artifact và managed source có error typed cục bộ, giữ diagnostic tiếng Anh ổn định. Lỗi filesystem/compiler/executor dùng chung giữ source chain. Test hiện ở `cli/src/test/build_test.rs`; S162 chưa chạy test.
