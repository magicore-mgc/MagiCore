#![cfg(test)]
#![allow(clippy::unwrap_used)]
//! Temporary filesystem fixture shared by scaffold owner tests.
//! Fixture filesystem tạm dùng chung bởi test các owner scaffold.

use tempfile::TempDir;

pub(crate) fn tmp() -> TempDir {
    TempDir::new().unwrap()
}
