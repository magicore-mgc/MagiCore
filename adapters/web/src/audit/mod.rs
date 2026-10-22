//! Audit module facade. / Facade module audit.

mod advisory_query;
mod audit;
mod audit_fix;
mod pin_source;
mod report;
pub(crate) mod types;
mod v4_pins;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

// COMPAT facade (S07/P03) — these bodies now live in `mgc-js-service`
// (moved verbatim, behavior byte-identical; only `pub(crate)` → `pub`).
// Re-exported here so every existing path (`crate::audit::*`,
// `install/fetch.rs`, tests) keeps working.
// Facade tương thích (S07/P03) — body thật nằm ở `mgc-js-service`
// (chuyển verbatim, hành vi giống hệt từng byte). Re-export tại đây để mọi
// đường dùng cũ vẫn chạy.
// Callers / Bên gọi: web-internal `install/fetch.rs` (tarball trust +
// loopback gate), `registry_config` guards (via `crate::audit::` — now
// re-exported from the service), the `run_audit` owner, `audit/test/advisory_compat`
// (parser + request-body units).
// TODO(S10-cleanup): remove this facade after S08 migrates the last callers.
pub use mgc_js_service::audit::{
    advisory_bulk_request_body, is_tarball_url_trusted, parse_advisory_bulk_response,
    registry_advisory_bulk_endpoint,
};
pub use mgc_js_service::registry::allow_insecure_loopback_url;

pub use audit::run_audit;
pub use audit_fix::run_audit_fix;
pub(crate) use v4_pins::v4_pins_for_audit;
