// Go advisory severity mapping tests.
// Kiểm thử ánh xạ severity advisory Go.

#[test]
fn severity_maps_vulndb_label_and_defaults_info() {
    let parsed = parse_govulncheck_json(VULNERABLE_STREAM).unwrap();
    let crypto = parsed
        .vulnerabilities
        .iter()
        .find(|v| v.cve == "CVE-2021-43565")
        .unwrap();
    // database_specific.severity=HIGH → High level, "high" label.
    // database_specific.severity=HIGH → level High, nhãn "high".
    assert_eq!(crypto.severity, "high");
    assert!(matches!(
        crypto.severity_level,
        mgc_types::adapter::VulnerabilitySeverity::High
    ));

    // GO-2020-0013 has no severity block → honest "info", no guessing.
    // GO-2020-0013 không có severity → "info" trung thực, không đoán.
    let text_vuln = parsed
        .vulnerabilities
        .iter()
        .find(|v| v.cve == "CVE-2020-14040")
        .unwrap();
    assert_eq!(text_vuln.severity, "info");
}

#[test]
fn cvss_vector_severity_fallback() {
    // No database_specific label → CVSS vector drives the level.
    // Không có nhãn database_specific → vector CVSS quyết định level.
    let stream = concat!(
        r#"{"osv":{"id":"GO-2023-0001","summary":"cvss driven","severity":[{"score":"CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:C/C:H/I:H/A:H"}]}}"#,
        "\n",
        r#"{"SBOM":{"modules":[{"path":"example.com/v","version":"v1.2.3"}]}}"#,
        "\n",
        r#"{"finding":{"osv":"GO-2023-0001","fixed_version":"v1.2.4","trace":[{"module":"example.com/v","version":"v1.2.3"}]}}"#,
        "\n",
    );
    let parsed = parse_govulncheck_json(stream).unwrap();
    let vuln = &parsed.vulnerabilities[0];
    assert!(matches!(
        vuln.severity_level,
        mgc_types::adapter::VulnerabilitySeverity::Critical
    ));
    assert_eq!(vuln.severity, "critical");
}
