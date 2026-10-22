# model_audit

EN: Keeps the public async `audit_model` entry point and selects the single-file or directory owner.

VI: Giữ entry point async public `audit_model` và chọn owner quét file hoặc thư mục.

- API: exported as `audit::audit_model`; missing-path error text and report path semantics are unchanged.
- Invariant: the model path is checked before a report is created, then the existing traversal is used.
- Errors: propagates `MgError`/`MgResult` without conversion or swallowed errors.
- Extension: entry orchestration lives here; extension matching and traversal stay in their owners.
- Tests: `test/missing_path.rs` checks the public missing-path error; no runtime test has run.
