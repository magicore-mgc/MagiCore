# CAS Open File Owner

**Purpose:** Map the shared platform no-follow opener into stable CAS diagnostics.

**Mục đích:** Ánh xạ helper no-follow của platform thành diagnostic CAS ổn định.

This owner keeps store-specific typed errors, symlink messages and `StoreError::Io` mapping out of the cross-platform primitive. `error.rs` preserves the local categories; `test/` checks stable diagnostics. The returned file handle is passed to hash and memo-identity paths without reopening the pathname.

Owner này giữ lỗi typed, message symlink riêng và mapping `StoreError::Io` ở ngoài primitive dùng chung đa nền tảng. `error.rs` giữ nhóm lỗi cục bộ; `test/` kiểm tra diagnostic ổn định. Handle được truyền tiếp cho hash/memo identity, không mở lại path.
