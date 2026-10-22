//! Checks dependency-spec parsing at its owner.
//! Kiểm tra parse dependency tại owner gần nhất.

use super::super::DependencySpec;

#[test]
fn parses_scoped_name_and_range() {
    let spec = DependencySpec::parse("@scope/pkg@^1.2.0").unwrap();
    assert_eq!(spec.name.as_str(), "@scope/pkg");
    assert_eq!(spec.range.as_str(), "^1.2.0");
    assert!(spec.default_features);
}

#[test]
fn rejects_empty_explicit_range() {
    assert!(DependencySpec::parse("pkg@").is_err());
}
