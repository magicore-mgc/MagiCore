// Advisory response parser contract tests. / Test hợp đồng parser response advisory.
#![allow(clippy::unwrap_used)]

use super::parse_advisory_bulk_response;

fn pins(entries: &[(&str, &str)]) -> Vec<(String, String)> {
    entries
        .iter()
        .map(|(name, version)| ((*name).to_string(), (*version).to_string()))
        .collect()
}

fn advisory(id: u64, vulnerable_versions: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "url": "https://github.com/advisories/GHSA-fixture",
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
