# Native Install tests

- `cli/src/commands/package_runtime/native_install/test/native_install.rs` — Current-MGC invocation options, install scope, and explicit lifecycle-script opt-in.
- Registered from `cli/src/commands/package_runtime/native_install/native_install.rs` through `#[path = "test/native_install.rs"]`.

- `cli/src/commands/package_runtime/native_install/test/native_install.rs` — Kiểm options gọi MGC hiện tại, install scope và lifecycle-script opt-in.
- Đăng ký tại `cli/src/commands/package_runtime/native_install/native_install.rs` qua `#[path = "test/native_install.rs"]`.

This owner propagates adjacent-owner and child-process errors unchanged, so it has no separate typed-error test. These unit tests do not replace dlx production-binary E2E and are currently `NOT_RUN`.
Owner propagate lỗi của owner liền kề và process con nguyên trạng nên không có test lỗi typed riêng. Test unit không thay dlx production-binary E2E và hiện là `NOT_RUN`.
