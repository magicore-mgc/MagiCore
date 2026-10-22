use crate::policy::invocation_key_for_tests;

// Profile key helper is stable for comparisons.
// (Helper key profile ổn định để so sánh.)
#[test]
fn profile_key_format() {
    assert_eq!(
        invocation_key_for_tests("npm", &["--version".to_string()]),
        "npm --version"
    );
    assert_eq!(invocation_key_for_tests("/usr/bin/npm", &[]), "npm");
}

#[test]
fn profile_membership_requires_the_full_normalized_invocation() {
    let profile = crate::policy::test::support::lane();
    assert!(crate::policy::profile_allows_invocation(
        &profile,
        "npm",
        &["--version".into()]
    ));
    assert!(!crate::policy::profile_allows_invocation(
        &profile,
        "npm",
        &["install".into()]
    ));
}
