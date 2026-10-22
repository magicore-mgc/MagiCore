# Run Registry Package tests

- `cli/src/commands/package_runtime/run_registry_package/test/run_registry_package.rs` — `test/run_registry_package.rs` checks real directory, regular-file and symlink cases; `test/error.rs` checks typed output.
- `cli/src/commands/package_runtime/run_registry_package/test/error.rs` — downcast type and stable English diagnostic for the owner-specific typed error.
- Registrations are in `run_registry_package/run_registry_package.rs` and `run_registry_package/error.rs` via explicit `#[path = "test/..."]` attributes.

- `cli/src/commands/package_runtime/run_registry_package/test/run_registry_package.rs` — test/run_registry_package.rs kiểm thư mục thật/file/symlink; test/error.rs kiểm output typed.
- `cli/src/commands/package_runtime/run_registry_package/test/error.rs` — kiểm type downcast và diagnostic tiếng Anh ổn định của lỗi typed thuộc owner.
- Đăng ký tại `run_registry_package/run_registry_package.rs` và `run_registry_package/error.rs` bằng `#[path = "test/..."]`.

These are unit claims only; they do not replace dlx production-binary E2E. The tests are currently `NOT_RUN`.
Đây chỉ là test unit, không thay dlx production-binary E2E. Hiện test là `NOT_RUN`.
