//! Wire resolved package projection tests.
//! Nối test projection package đã resolve.

#![allow(clippy::unwrap_used)]

use super::*;

#[path = "dependency_projection.rs"]
mod dependency_projection;
#[path = "duplicate_package_names.rs"]
mod duplicate_package_names;
mod support;
