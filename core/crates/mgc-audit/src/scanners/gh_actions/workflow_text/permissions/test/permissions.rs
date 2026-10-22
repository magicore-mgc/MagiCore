//! Test workflow permissions detection.
//! Kiểm tra nhận diện permissions workflow.
#![allow(clippy::unwrap_used)]

use super::super::{missing_finding, observe_top_level};

#[test]
fn only_indent_zero_permissions_counts_as_top_level() {
    let mut found = false;
    observe_top_level(2, "permissions:", &mut found);
    assert!(!found);
    observe_top_level(0, "permissions: read-all", &mut found);
    assert!(found);
}

#[test]
fn missing_permissions_text_and_identity_remain_stable() {
    let finding = missing_finding("ci.yml", false).unwrap();
    assert_eq!(finding.file, "ci.yml");
    assert_eq!(finding.line, 0);
    assert_eq!(finding.rule, "permissions-undeclared");
    assert_eq!(
        finding.detail,
        "workflow has no top-level `permissions:` block — defaults may grant write"
    );
    assert!(missing_finding("ci.yml", true).is_none());
}
