//! Directory audit test module wiring.
//! Wiring module test audit thư mục.

#![allow(clippy::unwrap_used)]

use crate::audit::audit_model;
use tempfile::TempDir;

fn tmp() -> TempDir {
    tempfile::tempdir().unwrap()
}

mod directory;
