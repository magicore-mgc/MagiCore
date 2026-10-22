// COMPAT facade (S07/P03) — real owners live in `mgc-js-service::manifest`.
// Moved verbatim (S07, behavior byte-identical): `parse_manifest`,
// `write_manifest` (+ `PackageJson`, atomic-write helpers,
// workspace-protocol guard).
// Facade tương thích (S07/P03) — owner thật nằm ở
// `mgc-js-service::manifest`. Đã chuyển verbatim (S07, hành vi giống hệt
// từng byte): `parse_manifest`, `write_manifest` (+ `PackageJson`, helper
// atomic-write, guard workspace-protocol).
// Callers / Bên gọi — web-internal: `list`, `update`, `audit` (fix + v4
// pins), `PackageAdapter`/`LockfileProvider` impls + `PackageJson`
// re-export (`lib.rs`), cache stamp writers (`cache.rs`, `cache_prune.rs`,
// `install/package_marker/`), unit tests (glob import); blessed-interim
// external (S06 §3/D014): `adapters/app` `manifest/react_native.rs`
// (retarget to `mgc-js-service` in S08).
// TODO(S10-cleanup): remove this facade after S08 migrates the last callers.
pub use mgc_js_service::manifest::*;
