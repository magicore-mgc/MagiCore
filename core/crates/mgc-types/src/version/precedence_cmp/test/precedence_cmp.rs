//! Exercises the precedence_cmp owner at its nearest test location.
//! Kiểm tra owner precedence_cmp tại vị trí test gần nhất.

use crate::version::test_support::v;
use std::cmp::Ordering;

#[test]
fn semver_build_metadata_is_preserved_without_changing_precedence() {
    let tagged = v("0.25.17+spec-1.1.0");
    assert_eq!(tagged.build.as_deref(), Some("spec-1.1.0"));
    assert_eq!(tagged.to_string(), "0.25.17+spec-1.1.0");
    assert_ne!(tagged, v("0.25.17"), "metadata is part of package identity");
    assert_eq!(
        tagged.precedence_cmp(&v("0.25.17")),
        Ordering::Equal,
        "semver requirements ignore build metadata"
    );
}
