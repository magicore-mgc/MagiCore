//! Package manifest entry-selection wiring.
//! Wiring chọn entry từ manifest package.

#[path = "exports_map/mod.rs"]
mod exports_map;
#[path = "manifest_entry.rs"]
mod implementation;

pub(in crate::bundler::dependency_cache::package_entry::implementation) use self::exports_map::{
    exports_root_target, exports_subpath_target, resolve_export_file,
};
pub(super) use self::implementation::resolve_manifest_entry;
