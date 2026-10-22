//! Package-index owner wiring.
//! Wiring owner dựng index package.

#[path = "package_index.rs"]
mod implementation;

pub(crate) use implementation::{
    OwnerPackageIndex, belongs_to_owner, build_owner_package_index, has_owner_packages,
    name_candidates, owner_core, owner_package_candidates, scoped_name_candidates,
    scoped_package_candidates,
};
