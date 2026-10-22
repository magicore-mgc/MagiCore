//! OWASP parser report findings cases.
//! Case OWASP parser: report findings.

#![allow(clippy::unwrap_used)]

use super::*;

#[test]
fn owasp_report_json_parses_cvss_severities() {
    let raw = r#"{
  "dependencies": [
    {
      "packages": [{"package": {"id": "pkg:maven/com.example/lib@1.0"}}],
      "vulnerabilities": [
        {
          "name": "CVE-2024-0001",
          "severity": "High",
          "cvssv3": {"baseScore": 8.1},
          "description": "RCE in lib"
        },
        {
          "name": "CVE-2024-0002",
          "severity": "Low",
          "description": "info leak"
        }
      ]
    },
    {"packages": [], "vulnerabilities": []}
  ]
}"#;
    let report = parse_owasp_dependency_check_json(raw).unwrap();
    assert_eq!(report.vulnerability_count, 2);
    assert_eq!(report.packages_audited, 2);
    let high = &report.vulnerabilities[0];
    assert_eq!(high.cve, "CVE-2024-0001");
    assert_eq!(
        high.severity_level,
        mgc_types::adapter::VulnerabilitySeverity::High
    );
    // Version must come from the purl — NOT a placeholder 0.0.0.
    // Version phải lấy từ purl — KHÔNG phải 0.0.0 giả.
    // (mgc Version normalizes "1.0" to 1.0.0 — the version IS carried.)
    // (Version mgc chuẩn hóa "1.0" thành 1.0.0 — version vẫn được giữ.)
    assert_eq!(high.package.version().to_string(), "1.0.0");
    assert_eq!(high.package.name_str(), "lib");
}

#[test]
fn owasp_report_purl_without_version_keeps_fallback() {
    // Some purls carry no @version — then 0.0.0 fallback is honest.
    // Một số purl không có @version — khi đó fallback 0.0.0 là trung thực.
    let raw = r#"{
  "dependencies": [
    {
      "packages": [{"package": {"id": "pkg:maven/com.example/no-ver"}}],
      "vulnerabilities": [
        {"name": "CVE-2024-0003", "severity": "Critical"}
      ]
    }
  ]
}"#;
    let report = parse_owasp_dependency_check_json(raw).unwrap();
    assert_eq!(report.vulnerability_count, 1);
    let v = &report.vulnerabilities[0];
    assert_eq!(v.package.version().to_string(), "0.0.0");
    assert_eq!(
        v.severity_level,
        mgc_types::adapter::VulnerabilitySeverity::Critical
    );
}

#[test]
fn owasp_report_missing_dependencies_key_fails_closed() {
    // No "dependencies" key → NOT a dependency-check report → schema error.
    // Thiếu key "dependencies" → không phải report dependency-check → lỗi.
    assert!(parse_owasp_dependency_check_json(r#"{}"#).is_err());
    assert!(parse_owasp_dependency_check_json("not json").is_err());
}

#[test]
fn owasp_report_utf8_description_truncates_on_char_boundary() {
    // 200+ multibyte chars must truncate on a char boundary — byte slicing
    // would panic mid-codepoint (external advisory data may be any UTF-8).
    // 200+ ký tự đa byte phải cắt đúng ranh giới ký tự — cắt byte sẽ panic
    // giữa codepoint (advisory ngoài có thể là UTF-8 bất kỳ).
    let long_desc = "gói bị ảnh hưởng nặng ".repeat(20);
    let raw = format!(
        r#"{{"dependencies": [{{"packages": [{{"package": {{"id": "pkg:maven/x/y@1.0"}}}}], "vulnerabilities": [{{"name": "CVE-1", "description": "{long_desc}"}}]}}]}}"#
    );
    let report = parse_owasp_dependency_check_json(&raw).unwrap();
    assert_eq!(report.vulnerability_count, 1);
    let title = &report.vulnerabilities[0].title;
    let desc_part = title.rsplit("CVE-1: ").next().unwrap();
    assert_eq!(desc_part.chars().count(), 200, "exactly 200 chars");
}
