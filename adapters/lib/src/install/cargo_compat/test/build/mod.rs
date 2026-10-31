// Shared imports for the Cargo build preflight tests.
// Import dùng chung cho test preflight build Cargo.
use super::super::*;
use crate::install::cargo_compat::test::Lockfile;
use crate::install::cargo_compat::test::{EcosystemTag, locked_package};

mod build;
