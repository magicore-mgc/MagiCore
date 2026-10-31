//! Node-style `exports` subpath map owner wiring.
//! Nối owner map subpath `exports` kiểu Node.

#[path = "exports_map.rs"]
mod implementation;

pub(in crate::bundler::dependency_cache::package_entry::implementation) use self::implementation::{
    exports_root_target, exports_subpath_target, resolve_export_file,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
