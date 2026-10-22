//! OWASP parser concurrent parse cases.
//! Case OWASP parser: concurrent parse.

#![allow(clippy::unwrap_used)]

use super::*;

#[test]
fn owasp_report_concurrent_parses_stay_independent() {
    // Parse song song payload sạch/nhiễu — không ghi chéo finding.
    // Concurrent parses of mixed payloads — no cross-contamination.
    let vulnerable = r#"{
  "dependencies": [
    {"packages": [{"package": {"id": "pkg:maven/a/b@1.0"}}],
     "vulnerabilities": [{"name": "CVE-2024-0001", "severity": "High"}]}
  ]
}"#;
    let clean = r#"{"dependencies": [{"packages": [], "vulnerabilities": []}]}"#;
    let payloads = [vulnerable, clean, vulnerable, clean, vulnerable];
    std::thread::scope(|scope| {
        let handles: Vec<_> = payloads
            .iter()
            .map(|p| {
                let payload = p.to_string();
                scope.spawn(move || parse_owasp_dependency_check_json(&payload).unwrap())
            })
            .collect();
        for (i, h) in handles.into_iter().enumerate() {
            let parsed = h.join().unwrap();
            let expected = if i % 2 == 0 { 1 } else { 0 };
            assert_eq!(
                parsed.vulnerability_count, expected,
                "concurrent parse {i} cross-contaminated"
            );
        }
    });
}
