# directory_audit

EN: Owns recursive traversal of a model directory and delegates each discovered file to `file_audit`.

VI: Quản lý duyệt đệ quy thư mục model và giao từng file thường cho `file_audit`.

- API: private to the `audit` module; public entry remains `audit::audit_model`.
- Existing caveat: `read_dir` entry errors are discarded with `flatten()`, and `Path::is_dir()` follows symlinks. This source-only move preserves that behavior.
- Security follow-up: add a separate containment/symlink/error-handling fix with dedicated tests before product readiness; this module is not a security approval.
- Errors: outer `read_dir` failures continue via `MgResult`; per-entry errors are currently ignored.
- Tests: `test/directory.rs` exercises the public audit route and is not run yet.
