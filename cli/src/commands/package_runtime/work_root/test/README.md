# Work Root tests

- `cli/src/commands/package_runtime/work_root/test/work_root.rs` — `test/work_root.rs` covers configured root, purpose separation, files and symlink chains; `test/error.rs` checks typed path context.
- `cli/src/commands/package_runtime/work_root/test/error.rs` — downcast type and stable English diagnostic for the owner-specific typed error.
- Registrations are in `work_root/work_root.rs` and `work_root/error.rs` via explicit `#[path = "test/..."]` attributes.

- `cli/src/commands/package_runtime/work_root/test/work_root.rs` — test/work_root.rs kiểm root, tách purpose, file và symlink; test/error.rs kiểm path context typed.
- `cli/src/commands/package_runtime/work_root/test/error.rs` — kiểm type downcast và diagnostic tiếng Anh ổn định của lỗi typed thuộc owner.
- Đăng ký tại `work_root/work_root.rs` và `work_root/error.rs` bằng `#[path = "test/..."]`.

These are unit claims only; they do not replace dlx production-binary E2E. The tests are currently `NOT_RUN`.
Đây chỉ là test unit, không thay dlx production-binary E2E. Hiện test là `NOT_RUN`.
