//! Checks conjunction behavior at the compound-range owner.
//! Kiểm tra conjunction tại owner compound range.

use super::super::super::match_compound_range::match_compound_range;
use crate::version::Version;

#[test]
fn requires_each_comparator_in_the_set() {
    let version = Version::parse("1.5.0").unwrap();
    assert!(match_compound_range(">=1.0.0 <2.0.0", &version));
    assert!(!match_compound_range(">=1.0.0 <1.4.0", &version));
}
