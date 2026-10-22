//! Package identifier and subpath selection wiring.
//! Wiring tách identifier package và chọn subpath.

#[path = "package_selector.rs"]
mod implementation;

pub(super) use self::implementation::{
    PackageSubpath, resolve_package_subpath, split_package_name_and_subpath,
};
