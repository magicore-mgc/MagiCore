//! Finding Class nearest-owner contract tests.
//! Test contract gần owner finding class.

use crate::adapter::*;

/// R4 (finding class): legacy payloads without `finding_class` must
/// deserialize as Vulnerability — schema stays backward compatible.
/// Finding không có `finding_class` phải về Vulnerability — schema cũ
/// vẫn đọc được.
#[test]
fn finding_class_defaults_to_vulnerability_for_legacy_json() {
    let legacy = serde_json::json!({
        "package": {"name": "lodash", "version": {"major": 4, "minor": 17, "patch": 12, "pre": null}},
        "title": "Prototype pollution",
        "severity": "high",
        "cve": "CVE-2019-10744",
        "severity_level": "high",
        "patched_versions": null,
        "url": null,
        "scanner": null,
        "ecosystem": null,
        "evidence_at": null
    });
    let vuln: Vulnerability = serde_json::from_value(legacy).unwrap();
    assert_eq!(vuln.finding_class, FindingClass::Vulnerability);
}

#[test]
fn finding_class_display_and_wire_names_match() {
    let cases = [
        (crate::adapter::FindingClass::Vulnerability, "vulnerability"),
        (crate::adapter::FindingClass::Policy, "policy"),
        (crate::adapter::FindingClass::Provenance, "provenance"),
        (crate::adapter::FindingClass::Artifact, "artifact"),
    ];
    for (class, wire) in cases {
        assert_eq!(class.as_str(), wire);
        assert_eq!(class.to_string(), wire);
        assert_eq!(
            serde_json::to_string(&class).unwrap(),
            format!("\"{wire}\"")
        );
    }
}
