# Digest decoding / Giải mã digest

Validates the `blake3-<base64>` prefix and decodes bytes shared by signature verification and signing. It does not hash or repair a supplied digest. Failures retain `LockfileError::VerificationFailed`.

Kiểm tra prefix `blake3-<base64>` và giải byte dùng chung cho verify/ký; không tự tính hash hay sửa digest. Lỗi giữ nguyên `VerificationFailed`.
