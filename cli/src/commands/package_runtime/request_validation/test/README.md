# Request Validation tests

- `cli/src/commands/package_runtime/request_validation/test/request_validation.rs` — `test/request_validation.rs` checks valid names/selectors and traversal/URL/control rejection; `test/error.rs` checks typed safe-reason handling.
- `cli/src/commands/package_runtime/request_validation/test/error.rs` — downcast type and stable English diagnostic for the owner-specific typed error.
- Registrations are in `request_validation/request_validation.rs` and `request_validation/error.rs` via explicit `#[path = "test/..."]` attributes.

- `cli/src/commands/package_runtime/request_validation/test/request_validation.rs` — test/request_validation.rs kiểm request hợp lệ và từ chối traversal/URL/control; test/error.rs kiểm reason an toàn và typed.
- `cli/src/commands/package_runtime/request_validation/test/error.rs` — kiểm type downcast và diagnostic tiếng Anh ổn định của lỗi typed thuộc owner.
- Đăng ký tại `request_validation/request_validation.rs` và `request_validation/error.rs` bằng `#[path = "test/..."]`.

These are unit claims only; they do not replace dlx production-binary E2E. The tests are currently `NOT_RUN`.
Đây chỉ là test unit, không thay dlx production-binary E2E. Hiện test là `NOT_RUN`.
