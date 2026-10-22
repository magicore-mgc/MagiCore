//! Checks wildcard lower and exclusive upper bounds.
//! Kiểm tra cận dưới và cận trên loại trừ của wildcard.

use super::super::super::parse_wildcard_bounds::parse_wildcard_bounds;

#[test]
fn expands_major_and_minor_wildcards() {
    let (low, high) = parse_wildcard_bounds("22.x").unwrap();
    assert_eq!(low.to_string(), "22.0.0");
    assert_eq!(high.to_string(), "23.0.0");
}
