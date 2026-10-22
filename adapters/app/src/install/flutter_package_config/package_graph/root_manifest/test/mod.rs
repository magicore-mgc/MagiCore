//! Wire root pubspec and dependency-name tests.
//! Nối test pubspec gốc và tên dependency.

#![allow(clippy::unwrap_used)]

use super::super::FlutterPackageGraphError;
use super::*;

#[path = "dependency_names.rs"]
mod dependency_names;
#[path = "root_manifest.rs"]
mod root_manifest;
