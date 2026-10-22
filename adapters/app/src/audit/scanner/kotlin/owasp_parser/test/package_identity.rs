//! OWASP parser package identity cases.
//! Case OWASP parser: package identity.

#![allow(clippy::unwrap_used)]

use super::*;

#[test]
fn owasp_report_hostile_purl_name_is_sanitized_to_artifact_fallback() {
    // A purl whose artifact tail cannot be a package name (traversal
    // dots, empty after qualifier strip) falls back to the SAFE
    // "artifact" name — the raw hostile string never reaches the
    // finding. A merely ODD but valid tail ("evil") passes through as a
    // display identifier (paths/spawns never derive from it).
    // Purl có đuôi artifact không thể là tên package (dấu chấm traversal,
    // rỗng sau khi lọc qualifier) thì rơi về tên an toàn "artifact" —
    // chuỗi hostile thô không bao giờ vào finding. Đuôi LẠ nhưng hợp lệ
    // ("evil") vẫn qua như định danh hiển thị (path/spawn không phái sinh
    // từ nó).
    let raw = r#"{
  "dependencies": [
    {
      "packages": [{"package": {"id": "pkg:maven/com.example/..@1.0.0"}}],
      "vulnerabilities": [{"name": "CVE-2024-0004", "severity": "Medium"}]
    }
  ]
}"#;
    let report = parse_owasp_dependency_check_json(raw).unwrap();
    assert_eq!(report.vulnerability_count, 1);
    let v = &report.vulnerabilities[0];
    assert_eq!(
        v.package.name_str(),
        "artifact",
        "traversal-dot purl must fall back to the safe artifact name"
    );
    assert_eq!(v.package.version().to_string(), "1.0.0");
}
