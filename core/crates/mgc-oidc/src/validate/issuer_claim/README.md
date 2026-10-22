# Unverified issuer claim

English: Decodes only the JWT payload `iss` field to choose a configured allowlisted issuer. This value is unauthenticated until `validate` verifies the signature and claims.

Tiếng Việt: Chỉ giải mã trường `iss` trong payload JWT để chọn issuer đã cấu hình trong allowlist. Giá trị này chưa được xác thực cho đến khi `validate` kiểm tra chữ ký và claim.

Nearest parser cases are in `test/`; they do not prove signature verification.
