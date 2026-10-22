# Export Permissions owner

Apply Unix executable semantics to project-side exports; other platforms retain the existing no-op.

Áp dụng executable mode Unix cho file export; platform khác giữ no-op như hiện tại.

Nearest source test: `test/export_permissions.rs`; it exercises this owner implementation directly. Runtime status: NOT_RUN.
