//! Checks primitive comparisons at the single-range owner.
//! Kiểm tra comparator đơn tại owner gần nhất.

use super::super::super::match_single_range::match_single_range;
use crate::version::Version;

#[test]
fn handles_zero_major_caret_and_comparison_operators() {
    let patch = Version::parse("0.2.8").unwrap();
    assert!(match_single_range("^0.2.3", &patch));
    assert!(!match_single_range("^0.3.0", &patch));
    assert!(match_single_range(
        ">1.0.0",
        &Version::parse("1.0.1").unwrap()
    ));
}
