# OIDC validation

English: This owner preserves the crate's public `mgc_oidc::validate::*` API and wires seven focused verification owners. `OidcError` remains the stable crate-wide error contract.

Tiếng Việt: Owner này giữ nguyên API công khai `mgc_oidc::validate::*` và nối các owner xác minh chuyên trách. `OidcError` tiếp tục là hợp đồng lỗi ổn định dùng chung toàn crate.

## Flow / Luồng

`validate_with_discovery` checks size and JWT header, reads `iss` only for exact allowlist selection, obtains cached JWKS, then calls full signature/claim validation. Discovery stays HTTPS-only and same-origin.

`validate_with_discovery` kiểm tra kích thước/header, chỉ đọc `iss` để chọn chính xác issuer trong allowlist, lấy JWKS cache rồi xác minh đầy đủ chữ ký/claim. Discovery chỉ HTTPS và cùng origin.

## Cache limit / Giới hạn cache

The process-static cache has at most 32 issuer slots and no eviction. Once full, a new issuer is rejected until process restart. This is bounded retention, not proof of zero memory cost; parsed JWKS and peak memory are not measured.

Cache tĩnh theo process có tối đa 32 slot issuer và không eviction. Khi đầy, issuer mới bị từ chối cho tới khi process khởi động lại. Đây là bộ nhớ giữ có giới hạn, không có nghĩa là không tốn bộ nhớ; chưa đo mức dùng sau parse JWKS.

Public API contract tests remain in `../../tests/validation.rs`; issuer payload parser has nearest-owner cases under `issuer_claim/test/`. Tests are authored/preserved but not run during the source-first phase.
