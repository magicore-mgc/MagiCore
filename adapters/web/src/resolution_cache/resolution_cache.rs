// Resolution cache keys for core-web — stable hashing of dependency inputs.
// Khóa cache resolution của core-web — tránh phụ thuộc thứ tự trong manifest.

use mgc_types::Manifest;

// COMPAT facade (S10a/P03) — real owner lives in `mgc-js-service::resolve`.
// Moved verbatim (S10a, behavior byte-identical): `manifest_resolution_cache_key`.
// Facade tương thích (S10a/P03) — owner thật nằm ở `mgc-js-service::resolve`.
// Callers / Bên gọi — web-internal: `resolve_inner` shared-cache key
// (`lib.rs`) + re-export (`lib.rs`); unit tests (`test/unit_tests.rs` five
// sites). Blessed-interim external: none (private cache key — S06 §3/D014
// external callers use manifest/audit lanes, retarget in S08).
// TODO(S10-cleanup): remove this facade after S10 migrates the last callers.
pub fn manifest_resolution_cache_key(manifest: &Manifest, registry_url: &str) -> String {
    mgc_js_service::resolve::manifest_resolution_cache_key(manifest, registry_url)
}
