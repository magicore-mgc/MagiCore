use super::super::*;

#[test]
fn nuget_range_semantics() {
    let v = Version::parse("1.2.3").expect("test operation must succeed");
    assert!(nuget_matches("1.2.3", &v), "bare = exact pin");
    assert!(!nuget_matches("1.2.4", &v));
    assert!(nuget_matches("*", &v));
    assert!(nuget_matches("1.*", &v));
    assert!(!nuget_matches("2.*", &v));
    assert!(nuget_matches("[1.2.3]", &v));
    assert!(nuget_matches("[1.0,2.0)", &v));
    assert!(!nuget_matches("[1.0,1.2.3)", &v));
}
