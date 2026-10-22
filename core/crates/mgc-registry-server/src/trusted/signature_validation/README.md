# Signature Validation

Decodes the shared signature wire format and validates npm SHA-512/PyPI SHA-256 inputs.

**Owned API:** `decode_signature, is_valid_sha512_integrity, is_valid_sha256_hex`. Imports are shared through the local trusted prelude; public names re-export to preserve `crate::trusted::*`.

# Signature Validation — Tiếng Việt

Giải mã wire signature dùng chung và kiểm tra đầu vào npm SHA-512/PyPI SHA-256.

**API sở hữu:** `decode_signature, is_valid_sha512_integrity, is_valid_sha256_hex`. Import dùng chung qua prelude nội bộ; tên công khai được export để giữ nguyên `crate::trusted::*`.
