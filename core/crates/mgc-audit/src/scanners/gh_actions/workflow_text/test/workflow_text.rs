//! Test workflow text scan orchestration and finding order.
//! Kiểm tra điều phối scan văn bản workflow và thứ tự finding.
#![allow(clippy::unwrap_used)]

use super::super::scan_workflow_text;

#[test]
fn scan_preserves_rule_order_and_line_numbers() {
    let raw = r#"on:
  pull_request_target:
jobs:
  check:
    steps:
      - uses: actions/checkout@v4
        run: echo ${{ github.event.pull_request.title }}
"#;
    let findings = scan_workflow_text("ci.yml", raw).unwrap();

    assert_eq!(findings.len(), 3);
    assert_eq!(findings[0].rule, "actions-unpinned-ref");
    assert_eq!(findings[0].line, 6);
    assert_eq!(findings[1].rule, "pull_request_target-injection");
    assert_eq!(findings[1].line, 7);
    assert_eq!(findings[2].rule, "permissions-undeclared");
    assert_eq!(findings[2].line, 0);
}

#[test]
fn top_level_permissions_suppresses_missing_permissions_finding() {
    let findings = scan_workflow_text("ci.yml", "permissions:\n  contents: read\n").unwrap();

    assert!(findings.is_empty());
}
