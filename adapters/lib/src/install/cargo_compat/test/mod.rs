//! Cargo compatibility tests grouped with their production owners.
//! Test Cargo compatibility được nhóm sát owner production.

use super::*;
pub(super) use mgc_lockfile::{EcosystemTag, Lockfile};
pub(super) use mgc_types::{PackageId, PackageName, ResolvedGraph, ResolvedPackage, Version};
pub(super) use sha2::Digest;

mod fixtures;
pub(super) use fixtures::{crate_archive, crate_archive_with_readme_case_aliases, locked_package};

mod build;
mod cargo_compat;
mod lock;
mod package_graph;
mod projection;
