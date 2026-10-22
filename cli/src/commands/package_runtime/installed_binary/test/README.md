# Installed Binary tests

- `cli/src/commands/package_runtime/installed_binary/test/installed_binary.rs` — `test/installed_binary.rs` covers owned targets and symlink escapes; `test/error.rs` checks diagnostic type/message contracts.
- `cli/src/commands/package_runtime/installed_binary/test/error.rs` — downcast type and stable English diagnostic for the owner-specific typed error.
- Registrations are in `installed_binary/installed_binary.rs` and `installed_binary/error.rs` via explicit `#[path = "test/..."]` attributes.

- `cli/src/commands/package_runtime/installed_binary/test/installed_binary.rs` — test/installed_binary.rs kiểm target thuộc package và symlink escape; test/error.rs kiểm type/message.
- `cli/src/commands/package_runtime/installed_binary/test/error.rs` — kiểm type downcast và diagnostic tiếng Anh ổn định của lỗi typed thuộc owner.
- Đăng ký tại `installed_binary/installed_binary.rs` và `installed_binary/error.rs` bằng `#[path = "test/..."]`.

These are unit claims only; they do not replace dlx production-binary E2E. The tests are currently `NOT_RUN`.
Đây chỉ là test unit, không thay dlx production-binary E2E. Hiện test là `NOT_RUN`.
