# JWT validation

English: Owns RS256-only signature verification, `kid` selection, issuer/audience/required-claim checks, expiry leeway and issued-at lifetime checks.

Tiếng Việt: Sở hữu xác minh chữ ký chỉ RS256, chọn `kid`, kiểm tra issuer/audience/claim bắt buộc, dung sai hết hạn và tuổi thọ token.

The public `validate` signature and `OidcError` categories/messages are preserved. The decoded identity is returned; token material is not returned. Public contract coverage remains in `mgc-oidc/tests/validation.rs`.
