# Tree digest framing / Định dạng ghép tree digest

Owns the byte framing for one normalized package file: path, NUL, decimal size, NUL, executable byte, NUL, file SHA-256, newline. Archive and extracted-tree owners use this shared implementation so they cannot drift independently.

Sở hữu cách ghép bytes cho một file package đã chuẩn hóa: path, NUL, size thập phân, NUL, byte executable, NUL, SHA-256 file, newline. Owner archive và extracted tree dùng chung để tránh lệch nhau.

Tests: `test/tree_digest.rs` asserts that path, size, and executable state each affect the digest. Errors and types are not introduced here because this owner is an infallible hash update.

Test: `test/tree_digest.rs` xác nhận path, size và executable state đều ảnh hưởng digest. Owner này chỉ cập nhật hash, không phát sinh error/type riêng.
