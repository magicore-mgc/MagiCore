use super::super::*;

#[test]
fn go_pin_semantics_and_operators() {
    let v = Version::parse("1.2.3").expect("test operation must succeed");
    // Bare version = exact pin (go.mod lock semantics).
    // (Version trần = ghim chính xác (ngữ nghĩa lock go.mod).)
    assert!(go_matches("1.2.3", &v));
    assert!(!go_matches("1.2.4", &v));
    assert!(go_matches("*", &v));
    assert!(go_matches(">=1.0, <2.0", &v));
    assert!(go_matches("^1.2", &v));
    assert!(!go_matches("^2.0", &v));
    assert!(go_matches("~1.2.0", &v));
    assert!(
        go_matches("v1.2.3", &v),
        "v-prefix on the range is tolerated"
    );
}
