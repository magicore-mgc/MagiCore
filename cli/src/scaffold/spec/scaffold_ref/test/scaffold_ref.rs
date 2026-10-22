//! Behavior tests for scaffold references.
//! Test hành vi cho reference scaffold.

use super::super::ScaffoldRef;

#[test]
fn validates_existing_dist_tags_and_default_reference() {
    assert!(
        ScaffoldRef::DistTag("latest".to_string())
            .validate()
            .is_ok()
    );
    assert!(ScaffoldRef::Default.validate().is_ok());
}

#[test]
fn rejects_empty_tag_and_known_typo_with_existing_messages() {
    assert_eq!(
        ScaffoldRef::DistTag(String::new())
            .validate()
            .unwrap_err()
            .to_string(),
        "Empty dist tag"
    );
    assert_eq!(
        ScaffoldRef::DistTag("laster".to_string())
            .validate()
            .unwrap_err()
            .to_string(),
        "Unknown tag 'laster'. Did you mean 'latest'?"
    );
}

#[test]
fn preserves_each_existing_dist_tag_typo_suggestion() {
    let cases = [
        ("laster", "latest"),
        ("stabl", "stable"),
        ("stabel", "stable"),
        ("betta", "beta"),
    ];

    for (tag, expected) in cases {
        assert_eq!(
            ScaffoldRef::DistTag(tag.to_string()).suggest_if_typo(),
            Some(expected.to_string())
        );
    }
}

#[test]
fn version_validation_preserves_the_leading_digit_heuristic() {
    assert_eq!(
        ScaffoldRef::Version(String::new())
            .validate()
            .unwrap_err()
            .to_string(),
        "Empty version"
    );
    assert!(
        ScaffoldRef::Version("1.not-full-semver".to_string())
            .validate()
            .is_ok()
    );
    assert_eq!(
        ScaffoldRef::Version("latest".to_string())
            .validate()
            .unwrap_err()
            .to_string(),
        "Version must start with digit, got 'latest'"
    );
}

#[test]
fn rejects_ranges_with_existing_message() {
    assert_eq!(
        ScaffoldRef::Range("^1.0.0".to_string())
            .validate()
            .unwrap_err()
            .to_string(),
        "Version ranges not supported yet"
    );
}
