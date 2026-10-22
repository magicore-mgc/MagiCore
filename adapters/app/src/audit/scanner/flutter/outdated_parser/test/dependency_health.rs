//! Flutter dependency-health and version-drift cases.
//! Case dependency-health và lệch version Flutter.

#![allow(clippy::unwrap_used)]

use super::*;

#[test]
fn flutter_outdated_reports_health_not_vulnerabilities() {
    let raw = r#"{
  "packages": [
    {"package": "http", "current": "0.13.3", "latest": "1.2.0", "kind": "direct"},
    {"package": "up_to_date_pkg", "current": "2.0.0", "latest": "2.0.0"},
    {"package": "no_latest", "current": "1.0.0", "latest": null}
  ],
  "devPackages": [
    {"package": "build_runner", "current": "2.0.0", "latest": "2.4.0"}
  ]
}"#;
    let health = parse_flutter_outdated_json(raw).unwrap();
    // All 4 entries were CHECKED — including up-to-date and latest-less.
    // Cả 4 entry đều ĐÃ KIỂM TRA — tính cả entry mới và không có latest.
    assert_eq!(health.packages_checked, 4);
    assert_eq!(health.outdated_count, 2, "http + build_runner are outdated");
    let first = &health.outdated[0];
    assert_eq!(first.package.name_str(), "http");
    assert_eq!(first.package.version().to_string(), "0.13.3");
    assert_eq!(first.latest_version.to_string(), "1.2.0");
    // pub outdated is NOT a security audit — this type carries no severity,
    // no CVE, and must never be coerced into a Vulnerability.
    // pub outdated KHÔNG phải security audit — type này không có severity,
    // không CVE, và không bao giờ được ép thành Vulnerability.
}

#[test]
fn flutter_outdated_all_current_is_zero_drift() {
    let raw = r#"{"packages": [{"package": "x", "current": "1.0.0", "latest": "1.0.0"}]}"#;
    let health = parse_flutter_outdated_json(raw).unwrap();
    assert_eq!(health.outdated_count, 0);
    assert_eq!(health.packages_checked, 1);
}

#[test]
fn flutter_outdated_dependency_overrides_group_counts() {
    // dependencyOverrides entries must be checked too — missing that group
    // would hide drift.
    // Entry dependencyOverrides cũng phải được kiểm tra — sót group này
    // là giấu lệch version.
    let raw = r#"{
  "packages": [],
  "dependencyOverrides": [
    {"package": "override_pkg", "current": "1.0.0", "latest": "2.0.0"}
  ]
}"#;
    let health = parse_flutter_outdated_json(raw).unwrap();
    assert_eq!(health.packages_checked, 1);
    assert_eq!(health.outdated_count, 1);
    assert_eq!(health.outdated[0].package.name_str(), "override_pkg");
}
