//! Existing policy refusal text compatibility tests.
//! Test tương thích nội dung refusal policy hiện có.

use super::super::SupplyChainError;
use mgc_types::Version;

#[test]
fn downgrade_keeps_the_existing_override_guidance() {
    let error = SupplyChainError::Downgrade {
        name: "widget".to_owned(),
        previous: Box::new(Version::parse("2.0.0").expect("parse previous version")),
        selected: Box::new(Version::parse("1.9.0").expect("parse selected version")),
    };

    assert!(error.to_string().contains("previous top-level 2.0.0"));
    assert!(error.to_string().contains("new top-level selection 1.9.0."));
    assert!(error.to_string().contains("MAGICORE_ALLOW_UNTRUSTED=1"));
}
