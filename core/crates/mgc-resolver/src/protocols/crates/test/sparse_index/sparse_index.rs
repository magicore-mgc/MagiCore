use super::super::*;

#[test]
fn sparse_prefix_rules() {
    assert_eq!(sparse_prefix("a"), "1");
    assert_eq!(sparse_prefix("ab"), "2");
    assert_eq!(sparse_prefix("abc"), "3/a");
    assert_eq!(sparse_prefix("serde"), "se/rd");
}
