//! Test the stable public workflow finding data shape.
//! Kiểm tra shape dữ liệu workflow finding public ổn định.
#![allow(clippy::unwrap_used)]

use super::WorkflowFinding;

#[test]
fn public_finding_fields_survive_clone() {
    let finding = WorkflowFinding {
        file: "ci.yml".to_string(),
        line: 12,
        rule: "actions-unpinned-ref",
        detail: "mutable ref".to_string(),
    };
    let cloned = finding.clone();

    assert_eq!(cloned.file, "ci.yml");
    assert_eq!(cloned.line, 12);
    assert_eq!(cloned.rule, "actions-unpinned-ref");
    assert_eq!(cloned.detail, "mutable ref");
}
