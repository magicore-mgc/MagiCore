// COMPAT facade (S07/P03) — real owners live in `mgc-js-service::registry`.
// Moved verbatim (S07, value/behavior byte-identical): `DEFAULT_NPM_REGISTRY`
// (R09/R14 compatibility surface — moved, NEVER changed),
// `effective_registry_url`, `validate_registry_allowed` (+ loopback helper
// `allow_insecure_loopback_url` from `audit`, needed by the guards).
// Facade tương thích (S07/P03) — owner thật nằm ở
// `mgc-js-service::registry`. Đã chuyển verbatim (S07, giá trị/hành vi
// giống hệt từng byte).
// Callers / Bên gọi — web-internal: `WebAdapter::{new,with_registry*,...}`
// constructors + scoped-chain guard (`lib.rs`); blessed-interim external
// (S06 §3/D014): `adapters/app` audit scanner via
// `mgc_web_adapter::DEFAULT_NPM_REGISTRY` (retarget in S08).
// TODO(S10-cleanup): remove this facade after S08 migrates the last callers.

pub use mgc_js_service::registry::{
    DEFAULT_NPM_REGISTRY, effective_registry_url, validate_registry_allowed,
};
