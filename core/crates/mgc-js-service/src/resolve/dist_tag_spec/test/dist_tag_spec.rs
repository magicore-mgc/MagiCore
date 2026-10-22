//! Alphabetic registry tags stay separate from semver-like specs.
//! Registry tag chữ cái vẫn tách khỏi spec giống semver.

use super::super::is_dist_tag_spec;

#[test]
fn accepts_alphabetic_tags_and_rejects_semver_forms() {
    assert!(is_dist_tag_spec("latest"));
    assert!(is_dist_tag_spec("next"));
    assert!(is_dist_tag_spec("beta"));
    assert!(!is_dist_tag_spec("*"));
    assert!(!is_dist_tag_spec(""));
    assert!(!is_dist_tag_spec("^1.2.3"));
    assert!(!is_dist_tag_spec("1.2.3"));
    assert!(!is_dist_tag_spec(">=1.0.0 <2.0.0"));
    assert!(!is_dist_tag_spec("abc123"));
}
