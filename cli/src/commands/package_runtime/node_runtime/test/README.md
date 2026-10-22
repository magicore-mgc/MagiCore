# Node Runtime tests

- `cli/src/commands/package_runtime/node_runtime/test/node_runtime.rs` — `test/node_runtime.rs` checks extensions, host lookup and quoted entrypoint paths; `test/error.rs` checks typed missing-runtime output.
- `cli/src/commands/package_runtime/node_runtime/test/error.rs` — downcast type and stable English diagnostic for the owner-specific typed error.
- Registrations are in `node_runtime/node_runtime.rs` and `node_runtime/error.rs` via explicit `#[path = "test/..."]` attributes.

- `cli/src/commands/package_runtime/node_runtime/test/node_runtime.rs` — test/node_runtime.rs kiểm extension, host lookup, quote path; test/error.rs kiểm lỗi typed.
- `cli/src/commands/package_runtime/node_runtime/test/error.rs` — kiểm type downcast và diagnostic tiếng Anh ổn định của lỗi typed thuộc owner.
- Đăng ký tại `node_runtime/node_runtime.rs` và `node_runtime/error.rs` bằng `#[path = "test/..."]`.

These are unit claims only; they do not replace dlx production-binary E2E. The tests are currently `NOT_RUN`.
Đây chỉ là test unit, không thay dlx production-binary E2E. Hiện test là `NOT_RUN`.
