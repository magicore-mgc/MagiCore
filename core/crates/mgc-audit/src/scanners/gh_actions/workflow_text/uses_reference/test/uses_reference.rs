//! Test action reference pin detection.
//! Kiểm tra phát hiện pin action reference.
#![allow(clippy::unwrap_used)]

use super::super::finding;

#[test]
fn mutable_action_ref_produces_the_existing_finding() {
    let finding = finding("ci.yml", 4, "uses: actions/checkout@v4").unwrap();
    assert_eq!(finding.rule, "actions-unpinned-ref");
    assert_eq!(finding.file, "ci.yml");
    assert_eq!(finding.line, 4);
    assert_eq!(
        finding.detail,
        "'actions/checkout@v4' is not pinned to a full 40-hex commit SHA — mutable refs can be hijacked"
    );
}

#[test]
fn full_sha_with_trailing_comment_is_accepted() {
    let finding = finding(
        "ci.yml",
        5,
        "uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4",
    );
    assert!(finding.is_none());
}

#[test]
fn quoted_full_sha_is_accepted() {
    let finding = finding(
        "ci.yml",
        6,
        "uses: 'actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683'",
    );
    assert!(finding.is_none());
}
