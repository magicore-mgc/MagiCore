// Advisory request contract tests. / Test hợp đồng request advisory.
#![allow(clippy::unwrap_used)]

use super::{advisory_bulk_request_body, registry_advisory_bulk_endpoint};

#[test]
fn builds_bulk_endpoint_from_registry_base() {
    assert_eq!(
        registry_advisory_bulk_endpoint("https://registry.npmjs.org/")
            .unwrap()
            .as_str(),
        "https://registry.npmjs.org/-/npm/v1/security/advisories/bulk"
    );
}

#[test]
fn rejects_malformed_registry_url() {
    assert!(registry_advisory_bulk_endpoint("not a URL").is_err());
}

#[test]
fn groups_every_unique_version_in_stable_order() {
    let pins = vec![
        ("zod".to_string(), "3.1.0".to_string()),
        ("lodash".to_string(), "4.17.21".to_string()),
        ("lodash".to_string(), "4.17.12".to_string()),
        ("lodash".to_string(), "4.17.12".to_string()),
    ];
    let body = advisory_bulk_request_body(&pins);
    assert_eq!(
        serde_json::Value::Object(body),
        serde_json::json!({
            "lodash": ["4.17.12", "4.17.21"],
            "zod": ["3.1.0"]
        })
    );
}
