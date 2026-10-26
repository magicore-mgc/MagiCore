//! The scan must catch injection sinks and stay quiet on clean output.
//! Scan phải bắt sink injection và yên với output sạch.

use super::super::{Severity, scan};

#[test]
fn a_clean_bundle_has_no_findings() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("app.js"),
        "console.log('hello');const x=eval('1+1');",
    )
    .unwrap();
    let report = scan(dir.path()).unwrap();
    assert!(report.findings.is_empty(), "{:?}", report.findings);
}

#[test]
fn eval_on_a_dynamic_argument_blocks() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("app.js"), "const y = eval(userInput);").unwrap();
    let report = scan(dir.path()).unwrap();
    assert!(report.has_blocking());
    assert!(report.findings.iter().any(|f| f.rule == "dynamic-eval"));
}

#[test]
fn a_development_build_is_noted_but_does_not_block() {
    // Shipping a dev build is a correctness problem, not an attack; it must be
    // visible without failing every build that has one.
    // Phát hành bản dev là vấn đề đúng đắn, không phải tấn công; nó phải hiện
    // ra mà không làm hỏng mọi build có nó.
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("app.js"),
        "/* react-dom.development */ console.log(1);",
    )
    .unwrap();
    let report = scan(dir.path()).unwrap();
    let finding = report
        .findings
        .iter()
        .find(|f| f.rule == "development-build")
        .expect("development build must be noted");
    assert_eq!(finding.severity, Severity::Note);
    assert!(!report.has_blocking());
}

#[test]
fn binary_assets_are_not_read_as_text() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("logo.png"), [0x89, 0x50, 0x4e, 0x47, 0x00]).unwrap();
    let report = scan(dir.path()).unwrap();
    assert!(report.findings.is_empty());
}
