# Bin Selection tests

- `cli/src/commands/package_runtime/bin_selection/test/bin_selection.rs` — `test/bin_selection.rs` protects name validation and deterministic selection; `test/error.rs` checks downcast variants and exact English messages.
- `cli/src/commands/package_runtime/bin_selection/test/error.rs` — downcast type and stable English diagnostic for the owner-specific typed error.
- Registrations are in `bin_selection/bin_selection.rs` and `bin_selection/error.rs` via explicit `#[path = "test/..."]` attributes.

- `cli/src/commands/package_runtime/bin_selection/test/bin_selection.rs` — test/bin_selection.rs kiểm tra tên và quy tắc chọn; test/error.rs kiểm variant downcast và message tiếng Anh.
- `cli/src/commands/package_runtime/bin_selection/test/error.rs` — kiểm type downcast và diagnostic tiếng Anh ổn định của lỗi typed thuộc owner.
- Đăng ký tại `bin_selection/bin_selection.rs` và `bin_selection/error.rs` bằng `#[path = "test/..."]`.

These are unit claims only; they do not replace dlx production-binary E2E. The tests are currently `NOT_RUN`.
Đây chỉ là test unit, không thay dlx production-binary E2E. Hiện test là `NOT_RUN`.
