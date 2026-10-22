# Install Policy tests

- `cli/src/commands/package_runtime/install_policy/test/install_policy.rs` — `test/install_policy.rs` checks script opt-in, inherited policy, redaction and native defaults; `test/error.rs` checks the typed failure.
- `cli/src/commands/package_runtime/install_policy/test/error.rs` — downcast type and stable English diagnostic for the owner-specific typed error.
- Registrations are in `install_policy/install_policy.rs` and `install_policy/error.rs` via explicit `#[path = "test/..."]` attributes.

- `cli/src/commands/package_runtime/install_policy/test/install_policy.rs` — test/install_policy.rs kiểm script opt-in, policy kế thừa, redaction và default; test/error.rs kiểm lỗi typed.
- `cli/src/commands/package_runtime/install_policy/test/error.rs` — kiểm type downcast và diagnostic tiếng Anh ổn định của lỗi typed thuộc owner.
- Đăng ký tại `install_policy/install_policy.rs` và `install_policy/error.rs` bằng `#[path = "test/..."]`.

These are unit claims only; they do not replace dlx production-binary E2E. The tests are currently `NOT_RUN`.
Đây chỉ là test unit, không thay dlx production-binary E2E. Hiện test là `NOT_RUN`.
