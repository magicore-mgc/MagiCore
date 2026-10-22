//! Tests the private helper directly from its owner subtree.
//! Kiểm tra helper private trực tiếp trong subtree owner.

use crate::version::Version;

#[test]
fn constructor_sets_release_fields_and_absent_metadata() {
    let version = Version::new(1, 2, 3);
    assert_eq!((version.major, version.minor, version.patch), (1, 2, 3));
    assert!(version.pre.is_none());
    assert!(version.build.is_none());
    assert!(version.dev.is_none());
    assert!(version.post.is_none());
}
