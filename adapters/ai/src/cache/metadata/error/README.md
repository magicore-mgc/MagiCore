# AI cache metadata errors

This owner distinguishes malformed JSON, symlink/non-regular sidecars, safe-open failures and operation-specific I/O. `MgError::Anyhow` retains the typed metadata error and its original source.

Owner này phân biệt JSON lỗi, sidecar symlink/không phải file thường, lỗi mở an toàn và I/O theo operation. `MgError::Anyhow` giữ lỗi metadata typed cùng source gốc.

Nearest source-chain cases are in `test/error.rs`.

Test source-chain gần owner nằm tại `test/error.rs`.
