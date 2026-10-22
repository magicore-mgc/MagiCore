//! `compat.rs` — Resolver compatibility exports / export tương thích resolver.
pub(crate) use mgc_js_service::resolve::{
    exact_version_from_range, is_manifest_root_package, manifest_with_resolved_specs,
    normalize_resolved_graph, select_resolved_candidate,
};
