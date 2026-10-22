//! Resolve canonical Python project and lock ownership.
//! Xác định core canonical sở hữu project Python và lock.

mod core_owner;
mod error;

pub(super) use core_owner::{
    canonical_core_name, python_project_owner, select_python_packages_for_owner,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
