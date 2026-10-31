// Advisory response parser contract tests. / Test hợp đồng parser response advisory.
#![allow(clippy::unwrap_used)]

use super::parse_advisory_bulk_response;
use crate::audit::response::is_cve_identifier;

fn pins(entries: &[(&str, &str)]) -> Vec<(String, String)> {
    entries
        .iter()
        .map(|(name, version)| ((*name).to_string(), (*version).to_string()))
        .collect()
}

fn advisory(id: u64, vulnerable_versions: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "url": "https://registry.npmjs.org/advisory/fixture",
        "title": "Fixture advisory",
        "severity": "high",
        "vulnerable_versions": vulnerable_versions,
    })
}

#[test]
fn projects_matching_locked_version_into_vulnerability() {
    let payload = serde_json::json!({"lodash": [advisory(17, "<4.17.21")]});
    let findings = parse_advisory_bulk_response(&payload, &pins(&[("lodash", "4.17.12")])).unwrap();
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].package.version().to_string(), "4.17.12");
    assert_eq!(findings[0].cve, "17");
}

#[test]
fn leaves_non_matching_locked_version_without_finding() {
    let payload = serde_json::json!({"lodash": [advisory(17, "<4.17.21")]});
    let findings = parse_advisory_bulk_response(&payload, &pins(&[("lodash", "4.17.21")])).unwrap();
    assert!(findings.is_empty());
}

#[test]
fn rejects_package_names_not_requested() {
    let payload = serde_json::json!({"other": [advisory(17, "<4.17.21")]});
    assert!(parse_advisory_bulk_response(&payload, &pins(&[("lodash", "4.17.12")])).is_err());
}

#[test]
fn rejects_malformed_top_level_and_advisory_shapes() {
    let requested = pins(&[("lodash", "4.17.12")]);
    assert!(parse_advisory_bulk_response(&serde_json::json!([]), &requested).is_err());
    assert!(
        parse_advisory_bulk_response(&serde_json::json!({"lodash": "not-an-array"}), &requested)
            .is_err()
    );
    assert!(
        parse_advisory_bulk_response(
            &serde_json::json!({"lodash": [{"id": "not-a-number"}]}),
            &requested
        )
        .is_err()
    );
}

#[test]
fn rejects_unparseable_vulnerable_ranges() {
    let payload = serde_json::json!({"lodash": [advisory(17, "not a semver range")]});
    assert!(parse_advisory_bulk_response(&payload, &pins(&[("lodash", "4.17.12")])).is_err());
}

/// Advisory carrying the real CVE array the npm bulk endpoint returns for
/// lodash@4.17.15 (the reported defect: only `1106913` was ever shown).
/// (Advisory mang mảng CVE thật mà bulk endpoint trả cho lodash@4.17.15
/// (lỗi đã báo: chỉ hiện `1106913`).)
fn advisory_with_cves(id: u64, cves: &[&str], url: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "url": url,
        "title": "Command Injection in lodash",
        "severity": "high",
        "vulnerable_versions": "<4.17.21",
        "cves": cves,
    })
}

#[test]
fn cves_array_surfaces_the_real_cve_identifier() {
    let payload = serde_json::json!({
        "lodash": [advisory_with_cves(
            1106913,
            &["CVE-2021-23337"],
            "https://github.com/advisories/GHSA-35jh-r3h4-6jhm",
        )]
    });
    let findings = parse_advisory_bulk_response(&payload, &pins(&[("lodash", "4.17.15")])).unwrap();
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].cve, "CVE-2021-23337");
}

#[test]
fn missing_cves_falls_back_to_the_ghsa_in_the_url() {
    let payload = serde_json::json!({
        "lodash": [serde_json::json!({
            "id": 1106913,
            "url": "https://github.com/advisories/GHSA-fixture",
            "title": "Fixture advisory",
            "severity": "high",
            "vulnerable_versions": "<4.17.21",
        })]
    });
    let findings = parse_advisory_bulk_response(&payload, &pins(&[("lodash", "4.17.15")])).unwrap();
    assert_eq!(findings[0].cve, "GHSA-fixture");
}

#[test]
fn empty_cves_falls_back_to_the_ghsa_in_the_url() {
    let payload = serde_json::json!({
        "lodash": [advisory_with_cves(
            1106913,
            &[],
            "https://github.com/advisories/GHSA-35jh-r3h4-6jhm",
        )]
    });
    let findings = parse_advisory_bulk_response(&payload, &pins(&[("lodash", "4.17.15")])).unwrap();
    assert_eq!(findings[0].cve, "GHSA-35jh-r3h4-6jhm");
}

#[test]
fn ghsa_only_cves_entry_is_used_before_the_numeric_id() {
    let payload = serde_json::json!({
        "lodash": [advisory_with_cves(
            1106913,
            &["GHSA-abcd-efgh-ijkl"],
            "https://example.invalid/advisory/1106913",
        )]
    });
    let findings = parse_advisory_bulk_response(&payload, &pins(&[("lodash", "4.17.15")])).unwrap();
    assert_eq!(findings[0].cve, "GHSA-abcd-efgh-ijkl");
}

#[test]
fn bare_numeric_id_stays_the_advisory_identifier() {
    let payload = serde_json::json!({
        "lodash": [advisory(1106913, "<4.17.21")]
    });
    let findings = parse_advisory_bulk_response(&payload, &pins(&[("lodash", "4.17.15")])).unwrap();
    assert_eq!(findings[0].cve, "1106913");
    assert!(
        !is_cve_identifier(&findings[0].cve),
        "a numeric id must never be treated as a CVE"
    );
}

#[test]
fn malformed_cve_entries_are_not_promoted_to_cve() {
    let payload = serde_json::json!({
        "lodash": [advisory_with_cves(
            1106913,
            &["CVE-21-23337", "not-a-cve", "CVE-2021-"],
            "https://example.invalid/advisory/1106913",
        )]
    });
    let findings = parse_advisory_bulk_response(&payload, &pins(&[("lodash", "4.17.15")])).unwrap();
    assert_eq!(findings[0].cve, "1106913");
}
