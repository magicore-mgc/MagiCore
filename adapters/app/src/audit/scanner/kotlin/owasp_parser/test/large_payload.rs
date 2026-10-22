//! OWASP parser large payload cases.
//! Case OWASP parser: large payload.

#![allow(clippy::unwrap_used)]

use super::*;

#[test]
fn owasp_report_large_payload_over_1mb_parses_all_findings() {
    // >1 MB report (nhiều dependency + finding) — parser tuyến tính,
    // không mất finding nào.
    // A >1 MB report — the parser is linear and loses no findings.
    let total = 6_500;
    let mut deps = String::new();
    for i in 0..total {
        let idx = format!("{i:04}");
        deps.push_str(&format!(
            r#"{{"packages": [{{"package": {{"id": "pkg:maven/g{idx}/a{idx}@1.0.{idx}"}}}}], "vulnerabilities": [{{"name": "CVE-2026-{idx}", "severity": "Low", "description": "d{idx}"}}]}},"#
        ));
    }
    deps.pop(); // drop the trailing comma
    let raw = format!(r#"{{"dependencies": [{deps}]}}"#);
    assert!(raw.len() > 1_000_000, "fixture must exceed 1 MB");
    let report = parse_owasp_dependency_check_json(&raw).unwrap();
    assert_eq!(report.vulnerability_count, total);
    assert_eq!(report.packages_audited, total);
}
