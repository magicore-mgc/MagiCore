//! Tests for manifest identity fallback and override behavior.
//! Test fallback và override định danh manifest.

use super::{name_or, version_or};

#[test]
fn empty_overrides_keep_manifest_identity() {
    assert_eq!(name_or("manifest-name", ""), "manifest-name");
    assert_eq!(version_or("1.0.0", ""), "1.0.0");
}

#[test]
fn non_empty_overrides_replace_manifest_identity() {
    assert_eq!(name_or("manifest-name", "override-name"), "override-name");
    assert_eq!(version_or("1.0.0", "2.0.0"), "2.0.0");
}
