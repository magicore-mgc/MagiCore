// Web manifest seed wiring — nối chọn version và cập nhật manifest.
// Re-export the crate API and focused internal seed operations.
use super::*;
#[path = "manifest_seed.rs"]
mod implementation;
pub(crate) use implementation::enrich_web_project_manifest;
#[cfg(test)]
pub(in crate::commands::core::web::implementation) use implementation::{
    ensure_primary_package, resolve_primary_version,
};
