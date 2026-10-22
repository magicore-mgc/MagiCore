//! Checks inclusive hyphen-range parsing.
//! Kiểm tra parse hyphen range inclusive.

use super::super::super::parse_hyphen_range::parse_hyphen_range;

#[test]
fn parses_three_part_hyphen_range() {
    let (low, high) = parse_hyphen_range("1.2.0 - 2.3.4").unwrap();
    assert_eq!(low.to_string(), "1.2.0");
    assert_eq!(high.to_string(), "2.3.4");
}
