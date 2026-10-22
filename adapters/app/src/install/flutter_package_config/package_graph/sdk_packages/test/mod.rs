//! Wire Flutter SDK package manifest/dependency tests.
//! Nối test manifest/dependency package Flutter SDK.

#![allow(clippy::unwrap_used)]

use super::super::FlutterPackageGraphError;
use super::*;
use std::collections::BTreeSet;

#[path = "sdk_package_projection.rs"]
mod sdk_package_projection;
#[path = "unsupported_dependency_source.rs"]
mod unsupported_dependency_source;
