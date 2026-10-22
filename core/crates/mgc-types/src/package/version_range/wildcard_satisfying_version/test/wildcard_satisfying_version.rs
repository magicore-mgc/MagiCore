//! Checks the current lower-bound estimate for wildcard ranges.
//! Kiểm tra ước lượng lower-bound hiện tại của wildcard range.

use super::super::super::wildcard_satisfying_version::wildcard_satisfying_version;

#[test]
fn returns_the_first_version_for_major_and_minor_wildcards() {
    assert_eq!(
        wildcard_satisfying_version("3.x").unwrap().to_string(),
        "3.0.0"
    );
    assert_eq!(
        wildcard_satisfying_version("3.4.x").unwrap().to_string(),
        "3.4.0"
    );
}
