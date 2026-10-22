# Trusted publishing

The public facade preserves `crate::trusted::*`. Functional owners separate protocol/package scope, OIDC builder identity, public DTOs, signing-key storage, attestation signing, signature validation, routes, and token/page-size policy. `mod.rs` only exports; `trusted.rs` wires owner modules and re-exports the established API.

# Trusted publishing — Tiếng Việt

Facade công khai giữ nguyên `crate::trusted::*`. Các owner tách protocol/package scope, OIDC builder identity, DTO công khai, lưu signing key, ký attestation, kiểm tra signature, route và policy token/page size. `mod.rs` chỉ export; `trusted.rs` nối owner và export lại API đã có.
