//! `advisory_compat.rs` — Web compatibility tests for advisory response parsing,
//! request-body grouping, and the explicit loopback policy re-export.
//! `advisory_compat.rs` — test tương thích Web cho parse advisory, gom request body,
//! và policy loopback được re-export tường minh.

#![allow(clippy::unwrap_used)]
#![allow(unsafe_code)]

use crate::audit::parse_advisory_bulk_response;

fn pins(entries: &[(&str, &str)]) -> Vec<(String, String)> {
    entries
        .iter()
        .map(|(n, v)| (n.to_string(), v.to_string()))
        .collect()
}

fn advisory(id: u64, vulnerable: &str, severity: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "url": "https://github.com/advisories/GHSA-test",
        "title": "Test advisory",
        "severity": severity,
        "vulnerable_versions": vulnerable,
    })
}

#[test]
fn lane_npm_1_clean_response_zero_findings() {
    // No advisories returned → clean (the packages were asked, the
    // registry answered none apply).
    // Không advisory trả về → sạch (package đã hỏi, registry trả lời
    // không cái nào áp dụng).
    let payload = serde_json::json!({});
    let vulns = parse_advisory_bulk_response(&payload, &pins(&[("lodash", "4.17.21")])).unwrap();
    assert!(vulns.is_empty());
}

#[test]
fn lane_npm_2_vulnerable_version_matches_range() {
    let payload = serde_json::json!({"lodash": [advisory(1102260, "<4.17.21", "high")]});
    let vulns = parse_advisory_bulk_response(&payload, &pins(&[("lodash", "4.17.12")])).unwrap();
    assert_eq!(vulns.len(), 1);
    assert_eq!(vulns[0].package.version().to_string(), "4.17.12");
    assert_eq!(vulns[0].cve, "1102260");
    assert_eq!(vulns[0].severity, "high");
    assert_eq!(vulns[0].scanner.as_deref(), Some("npm-bulk-advisory"));
    assert_eq!(vulns[0].ecosystem.as_deref(), Some("web/javascript"));
    assert!(vulns[0].evidence_at.is_some());
}

#[test]
fn lane_npm_3_patched_version_outside_range_no_finding() {
    let payload = serde_json::json!({"lodash": [advisory(1102260, "<4.17.21", "high")]});
    let vulns = parse_advisory_bulk_response(&payload, &pins(&[("lodash", "4.17.21")])).unwrap();
    assert!(vulns.is_empty(), "patched version must not match");
}

#[test]
fn lane_npm_4_malformed_response_fails_closed() {
    // Wrong shape, malformed advisory records, unparseable ranges — every
    // one is an error, never an implicit clean.
    // Shape sai, bản ghi advisory rác, range không parse được — tất cả
    // là lỗi, không bao giờ tự biến thành sạch.
    let p = &pins(&[("lodash", "4.17.12")]);
    for bad in [
        serde_json::json!([]),
        serde_json::json!("nope"),
        serde_json::Value::Null,
        serde_json::json!({"lodash": "not-array"}),
        serde_json::json!({"lodash": [serde_json::json!({"id": "string-not-number"})]}),
        serde_json::json!({"lodash": [serde_json::json!({"id": 1, "url": "u", "title": "t", "severity": "s", "vulnerable_versions": "garbage-range"})]}),
    ] {
        assert!(
            parse_advisory_bulk_response(&bad, p).is_err(),
            "malformed response must fail closed: {bad}"
        );
    }
}

#[test]
fn lane_npm_5_hostile_package_in_response_is_rejected() {
    // The response mentions a package we never requested — possible
    // hostile injection; hard error.
    // Response nhắc package ta chưa từng hỏi — có thể tiêm thù địch;
    // lỗi cứng.
    let payload = serde_json::json!({"evil-pkg": [advisory(1, "<99.0.0", "critical")]});
    let result = parse_advisory_bulk_response(&payload, &pins(&[("lodash", "4.17.12")]));
    assert!(result.is_err());
}

#[test]
fn lane_npm_6_semver_range_edges() {
    let p = &pins(&[("semver-edge", "2.0.0")]);
    // Exact-match style ranges from the real API.
    // Range kiểu khớp chính xác từ API thật.
    let exact = serde_json::json!({"semver-edge": [advisory(1, "=2.0.0", "medium")]});
    assert_eq!(
        parse_advisory_bulk_response(&exact, p).unwrap().len(),
        1,
        "=2.0.0 must match 2.0.0"
    );

    let above = serde_json::json!({"semver-edge": [advisory(2, "<2.0.0", "low")]});
    assert!(
        parse_advisory_bulk_response(&above, p).unwrap().is_empty(),
        "<2.0.0 must not match 2.0.0"
    );
}

#[test]
fn lane_npm_7_scoped_and_unicode_packages() {
    // @scope names and unicode titles ride through without mojibake.
    // Tên @scope và title unicode qua nguyên vẹn không vỡ chữ.
    let payload = serde_json::json!({
        "@babel/core": [serde_json::json!({
            "id": 7, "url": "https://x", "title": "RCE trong 📦 @babel/core — lỗi unicode",
            "severity": "critical", "vulnerable_versions": "<7.24.0"
        })]
    });
    let vulns =
        parse_advisory_bulk_response(&payload, &pins(&[("@babel/core", "7.23.9")])).unwrap();
    assert_eq!(vulns.len(), 1);
    assert_eq!(vulns[0].package.name().as_str(), "@babel/core");
    assert!(vulns[0].title.contains("📦"));
}

#[test]
fn lane_npm_8_large_response_over_1mb() {
    let total = 12_000;
    let mut map = serde_json::Map::new();
    for i in 0..total {
        map.insert(
            format!("pkg-{i:05}"),
            serde_json::Value::Array(vec![advisory(i as u64 + 1, "<1.0.0", "moderate")]),
        );
    }
    let payload = serde_json::Value::Object(map);
    let mut pin_entries = Vec::with_capacity(total);
    for i in 0..total {
        pin_entries.push((format!("pkg-{i:05}"), "0.9.0".to_string()));
    }
    let vulns = parse_advisory_bulk_response(&payload, &pin_entries).unwrap();
    assert_eq!(vulns.len(), total, "every advisory row must survive");
}

#[test]
fn lane_npm_9_multiple_installed_versions_per_package() {
    // Two pinned versions of the SAME package: one inside the range, one
    // outside → exactly one finding for the vulnerable pin.
    // Hai version ghim của CÙNG package: một trong range, một ngoài →
    // đúng một finding cho ghim dính lỗi.
    let payload = serde_json::json!({"lodash": [advisory(1102260, "<4.17.21", "high")]});
    let vulns = parse_advisory_bulk_response(
        &payload,
        &pins(&[("lodash", "4.17.12"), ("lodash", "4.17.21")]),
    )
    .unwrap();
    assert_eq!(vulns.len(), 1);
    assert_eq!(vulns[0].package.version().to_string(), "4.17.12");
}

#[test]
fn lane_npm_bulk_request_keeps_all_versions_for_each_package() {
    // The registry request must include every installed version of a name.
    // Request gửi registry phải giữ đủ mọi version đang cài của cùng tên.
    let body = crate::audit::advisory_bulk_request_body(&pins(&[
        ("lodash", "4.17.21"),
        ("react", "18.2.0"),
        ("lodash", "4.17.12"),
        ("lodash", "4.17.12"),
    ]));
    assert_eq!(
        serde_json::Value::Object(body),
        serde_json::json!({
            "lodash": ["4.17.12", "4.17.21"],
            "react": ["18.2.0"]
        })
    );
}

#[test]
fn lane_npm_10_concurrent_parses_no_cross_contamination() {
    let vulnerable = serde_json::json!({"lodash": [advisory(1102260, "<4.17.21", "high")]});
    let clean = serde_json::json!({});
    let payloads = [
        vulnerable.clone(),
        clean.clone(),
        vulnerable.clone(),
        clean.clone(),
    ];
    let pin_sets: Vec<Vec<(String, String)>> = payloads
        .iter()
        .map(|_| pins(&[("lodash", "4.17.12")]))
        .collect();
    std::thread::scope(|scope| {
        let handles: Vec<_> = payloads
            .iter()
            .zip(&pin_sets)
            .map(|(p, pins)| {
                let payload = p.clone();
                let pins = pins.clone();
                scope.spawn(move || parse_advisory_bulk_response(&payload, &pins).unwrap())
            })
            .collect();
        for (i, h) in handles.into_iter().enumerate() {
            let expected = if i % 2 == 0 { 1 } else { 0 };
            assert_eq!(
                h.join().unwrap().len(),
                expected,
                "concurrent parse {i} cross-contaminated"
            );
        }
    });
}

#[test]
fn insecure_loopback_requires_explicit_opt_in() {
    // Default deny: loopback http is rejected unless the operator opts in.
    // Serialized: env is process-global.
    let _guard = crate::environment_test_lock().lock().unwrap();
    let old = std::env::var_os("MAGICORE_WEB_ALLOW_INSECURE_LOCALHOST");
    let restore = |previous: Option<std::ffi::OsString>| {
        if let Some(value) = previous {
            unsafe { std::env::set_var("MAGICORE_WEB_ALLOW_INSECURE_LOCALHOST", value) };
        } else {
            unsafe { std::env::remove_var("MAGICORE_WEB_ALLOW_INSECURE_LOCALHOST") };
        }
    };
    unsafe { std::env::remove_var("MAGICORE_WEB_ALLOW_INSECURE_LOCALHOST") };
    assert!(!crate::audit::allow_insecure_loopback_url(
        "http://127.0.0.1:4315/x"
    ));
    assert!(!crate::audit::allow_insecure_loopback_url(
        "https://registry.npmjs.org/x"
    ));
    unsafe { std::env::set_var("MAGICORE_WEB_ALLOW_INSECURE_LOCALHOST", "1") };
    assert!(crate::audit::allow_insecure_loopback_url(
        "http://127.0.0.1:4315/x"
    ));
    assert!(!crate::audit::allow_insecure_loopback_url(
        "http://example.com/x"
    ));
    restore(old);
}
