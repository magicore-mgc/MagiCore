# Installed Metadata tests

- `cli/src/commands/package_runtime/installed_metadata/test/installed_metadata.rs` — `test/installed_metadata.rs` covers package ownership, semver and symlink refusal; `test/error.rs` checks the local typed variant.
- `cli/src/commands/package_runtime/installed_metadata/test/error.rs` — downcast type and stable English diagnostic for the owner-specific typed error.
- Registrations are in `installed_metadata/installed_metadata.rs` and `installed_metadata/error.rs` via explicit `#[path = "test/..."]` attributes.

- `cli/src/commands/package_runtime/installed_metadata/test/installed_metadata.rs` — test/installed_metadata.rs kiểm ownership, semver, symlink; test/error.rs kiểm variant.
- `cli/src/commands/package_runtime/installed_metadata/test/error.rs` — kiểm type downcast và diagnostic tiếng Anh ổn định của lỗi typed thuộc owner.
- Đăng ký tại `installed_metadata/installed_metadata.rs` và `installed_metadata/error.rs` bằng `#[path = "test/..."]`.

These are unit claims only; they do not replace dlx production-binary E2E. The tests are currently `NOT_RUN`.
Đây chỉ là test unit, không thay dlx production-binary E2E. Hiện test là `NOT_RUN`.
