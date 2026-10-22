use super::super::*;
use mgc_types::Version;

#[test]
fn cargo_caret_semantics() {
    let v = Version::parse("1.2.5").expect("test operation must succeed");
    assert!(cargo_matches("^1.2.3", &v));
    assert!(!cargo_matches(
        "^1.2.3",
        &Version::parse("2.0.0").expect("test operation must succeed")
    ));
    assert!(cargo_matches("1.2", &v)); // bare = caret
    assert!(cargo_matches(">=1.0, <2.0", &v));
    assert!(cargo_matches("~1.2.3", &v));
    assert!(!cargo_matches(
        "~1.2.3",
        &Version::parse("1.3.0").expect("test operation must succeed")
    ));
    assert!(cargo_matches("=1.2.5", &v));
    assert!(cargo_matches(
        "^0.25.0",
        &Version::parse("0.25.17+spec-1.1.0").expect("test operation must succeed")
    ));
}
