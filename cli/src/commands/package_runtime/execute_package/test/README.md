# Execute Package tests

- `cli/src/commands/package_runtime/execute_package/test/execute_package.rs` — PATH ordering for native executables and Node scripts.
- Registered from `cli/src/commands/package_runtime/execute_package/execute_package.rs` through `#[path = "test/execute_package.rs"]`.

- `cli/src/commands/package_runtime/execute_package/test/execute_package.rs` — Kiểm thứ tự PATH cho native executable và Node script.
- Đăng ký tại `cli/src/commands/package_runtime/execute_package/execute_package.rs` qua `#[path = "test/execute_package.rs"]`.

This owner propagates adjacent-owner and child-process errors unchanged, so it has no separate typed-error test. These unit tests do not replace dlx production-binary E2E and are currently `NOT_RUN`.
Owner propagate lỗi của owner liền kề và process con nguyên trạng nên không có test lỗi typed riêng. Test unit không thay dlx production-binary E2E và hiện là `NOT_RUN`.
