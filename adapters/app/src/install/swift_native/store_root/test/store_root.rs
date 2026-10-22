//! Behavior tests for the store_root owner.
//! Test hành vi cho owner store_root.

use super::super::*;
use std::path::{Path, PathBuf};

#[test]
fn empty_or_whitespace_override_uses_the_shared_default_root() {
    assert_eq!(override_store_root(None), None);
    assert_eq!(override_store_root(Some("  \t".into())), None);
    assert_eq!(
        default_store_root(Path::new("/data/magicore")),
        PathBuf::from("/data/magicore/store/swift")
    );
}

#[test]
fn non_empty_override_preserves_the_supplied_path() {
    assert_eq!(
        override_store_root(Some("/custom/swift-store".into())),
        Some(PathBuf::from("/custom/swift-store"))
    );
}
