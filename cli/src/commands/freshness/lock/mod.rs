//! Lock owner exports — export owner lock.

mod lock;
pub(super) use lock::{
    check_declared_roots, check_manifest_roots, locked_packages, locked_root_versions,
};
