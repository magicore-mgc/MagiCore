//! Package entry resolver wiring.
//! Wiring bộ phân giải entry package.

#[path = "allowed_roots/mod.rs"]
mod allowed_roots;
#[path = "error/mod.rs"]
mod error;
#[path = "package_entry.rs"]
mod implementation;

pub(super) use self::allowed_roots::allowed_dependency_roots;
pub(super) use self::error::PackageEntryError;
pub(super) use self::implementation::resolve_package_entry;
