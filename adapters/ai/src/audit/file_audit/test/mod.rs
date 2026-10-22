//! File audit test module wiring.
//! Wiring module test audit file.

#![allow(clippy::unwrap_used)]

use crate::audit::audit_model;
use tempfile::TempDir;

fn tmp() -> TempDir {
    tempfile::tempdir().unwrap()
}

mod single_file;
