//! Exercises the display owner at its nearest test location.
//! Kiểm tra owner display tại vị trí test gần nhất.

use crate::version::test_support::v;
#[test]
fn display_no_pre() {
    assert_eq!(v("1.2.3").to_string(), "1.2.3");
}

#[test]
fn display_with_pre() {
    assert_eq!(v("1.2.3-alpha").to_string(), "1.2.3-alpha");
}
