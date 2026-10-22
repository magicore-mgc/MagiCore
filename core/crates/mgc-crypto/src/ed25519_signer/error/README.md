# Ed25519 local errors / Lỗi local Ed25519

Typed failures retain the Base64 decode source and distinguish invalid lengths, key parsing and verification. Conversion maps back to the established crate-level `CryptoError` variants and public messages; no retries occur here.

Lỗi có kiểu giữ source decode Base64 và phân biệt độ dài sai, parse key và verify. Chuyển đổi ánh xạ về đúng variant/message `CryptoError` cấp crate; owner này không retry.
