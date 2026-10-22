// audit_tests — split from unit_tests.rs by behavior/subject (P07).
// audit_tests — tách từ unit_tests.rs theo behavior/chủ đề (P07).
use super::*;

#[tokio::test]
async fn test_audit_fix_bumps_vulnerable_packages_and_rewrites_lockfile() {
    let shared = tempdir_real().unwrap();
    let dir = tempdir_real().unwrap();

    seed_shared_metadata(
        shared.path(),
        "react",
        serde_json::json!({
            "name": "react",
            "description": null,
            "versions": {
                "18.2.0": {
                    "version": "18.2.0",
                    "dependencies": null,
                    "optionalDependencies": null,
                    "os": null,
                    "cpu": null,
                    "dist": {
                        "tarball": "https://registry.example.test/react-18.2.0.tgz",
                        "integrity": "sha512-c2hhcmVk"
                    }
                },
                "19.0.0": {
                    "version": "19.0.0",
                    "dependencies": null,
                    "optionalDependencies": null,
                    "os": null,
                    "cpu": null,
                    "dist": {
                        "tarball": "https://registry.example.test/react-19.0.0.tgz",
                        "integrity": "sha512-c2hhcmVk2"
                    }
                }
            },
            "dist-tags": { "latest": "19.0.0" }
        }),
    );

    std::fs::write(
        dir.path().join("package.json"),
        serde_json::json!({
            "name": "demo",
            "version": "0.1.0",
            "private": true,
            "dependencies": { "react": "^18.2.0" }
        })
        .to_string(),
    )
    .unwrap();

    let adapter = WebAdapter::with_registry_and_shared_cache(
        "http://127.0.0.1:9".into(),
        shared.path().to_path_buf(),
    );
    let vulnerable = vec![PackageId::new(
        PackageName::new("react").unwrap(),
        Version::parse("18.2.0").unwrap(),
    )];

    let bumped = adapter.audit_fix(dir.path(), &vulnerable).await.unwrap();
    assert_eq!(bumped, 1);

    let manifest = adapter.parse_manifest(dir.path()).await.unwrap();
    let dep = manifest.find_dep("react").unwrap();
    assert_eq!(dep.range.as_str(), "*");

    let lock = std::fs::read_to_string(dir.path().join("mgc.lock")).unwrap();
    let parsed: Lockfile = mgc_lockfile::parse_lockfile(&lock).unwrap();
    assert!(
        parsed
            .packages
            .iter()
            .any(|p| p.name == "react" && p.version == "19.0.0")
    );
}

#[tokio::test]
async fn test_audit_fix_fail_closed_keeps_manifest_and_lockfile_when_resolve_fails() {
    let shared = tempdir_real().unwrap();
    let dir = tempdir_real().unwrap();

    std::fs::write(
        dir.path().join("package.json"),
        serde_json::json!({
            "name": "demo",
            "version": "0.1.0",
            "private": true,
            "dependencies": { "react": "^18.2.0" }
        })
        .to_string(),
    )
    .unwrap();
    std::fs::write(
        dir.path().join("mgc.lock"),
        "[resolution]\nstate = \"locked\"\n",
    )
    .unwrap();
    let original_manifest = std::fs::read_to_string(dir.path().join("package.json")).unwrap();
    let original_lock = std::fs::read_to_string(dir.path().join("mgc.lock")).unwrap();

    // Registry + metadata cache both empty -> resolve must fail -> nothing written.
    let adapter = WebAdapter::with_registry_and_shared_cache(
        "http://127.0.0.1:9".into(),
        shared.path().to_path_buf(),
    );
    let vulnerable = vec![PackageId::new(
        PackageName::new("react").unwrap(),
        Version::parse("18.2.0").unwrap(),
    )];

    assert!(adapter.audit_fix(dir.path(), &vulnerable).await.is_err());
    assert_eq!(
        std::fs::read_to_string(dir.path().join("package.json")).unwrap(),
        original_manifest
    );
    assert_eq!(
        std::fs::read_to_string(dir.path().join("mgc.lock")).unwrap(),
        original_lock
    );
}

#[test]
fn test_audit_parse_builds_findings_from_vulnerable_versions() {
    // Installed 4.17.12 matches "<4.17.21" -> finding; installed 4.17.21
    // does NOT match -> no second finding. Client-side matching, the
    // pnpm model.
    // 4.17.12 khớp "<4.17.21" -> có finding; 4.17.21 KHÔNG khớp -> không
    // có finding thứ hai. Đối chiếu phía client theo mô hình pnpm.
    let lock = audit_lockfile_fixture();
    let payload = serde_json::json!({"lodash": [lodash_advisory_json()]});
    let vulns = parse_advisory_bulk_response(&payload, &lock).unwrap();
    assert_eq!(
        vulns.len(),
        1,
        "only the vulnerable installed version yields a finding"
    );
    assert_eq!(vulns[0].package.version().to_string(), "4.17.12");
    assert_eq!(vulns[0].cve, "1102260");
    assert_eq!(
        vulns[0].severity_level,
        mgc_types::adapter::VulnerabilitySeverity::High
    );
    assert!(
        vulns[0]
            .patched_versions
            .as_deref()
            .is_some_and(|p| p.contains("4.17.21"))
    );
}

#[test]
fn test_audit_parse_advisory_not_matching_installed_version_is_not_finding() {
    // Advisory range that does not cover any installed version: zero
    // findings is LEGITIMATE here (advisory not applicable), not clean-fake.
    // Range advisory không phủ version nào đang cài: 0 finding là HỢP LỆ
    // (advisory không áp dụng), không phải sạch giả.
    let lock = audit_lockfile_fixture();
    let advisory = serde_json::json!({
        "id": 1102261,
        "url": "https://example.com/a",
        "title": "Some other lodash issue",
        "severity": "moderate",
        "vulnerable_versions": ">=5.0.0 <6.0.0"
    });
    let payload = serde_json::json!({"lodash": [advisory]});
    let vulns = parse_advisory_bulk_response(&payload, &lock).unwrap();
    assert!(vulns.is_empty());
}

#[test]
fn test_audit_parse_empty_object_is_clean() {
    // No advisories for any requested package — the npm bulk endpoint
    // returns {} when nothing is vulnerable. That IS a clean result.
    // Không advisory nào — npm bulk endpoint trả {} khi không package dính.
    let lock = audit_lockfile_fixture();
    let vulns = parse_advisory_bulk_response(&serde_json::json!({}), &lock).unwrap();
    assert!(vulns.is_empty());
}

#[test]
fn test_audit_parse_rejects_array_response() {
    // A top-level array (or string, or null) violates the map contract —
    // must be an error, never an implicit clean.
    // Array top-level (hoặc string/null) vi phạm hợp đồng map — phải
    // lỗi, không được âm thầm sạch.
    let lock = audit_lockfile_fixture();
    assert!(parse_advisory_bulk_response(&serde_json::json!([]), &lock).is_err());
    assert!(parse_advisory_bulk_response(&serde_json::json!("error"), &lock).is_err());
    assert!(parse_advisory_bulk_response(&serde_json::Value::Null, &lock).is_err());
}

#[test]
fn test_audit_parse_rejects_unrequested_package() {
    // The response may only mention packages we asked about — anything
    // else is a malformed or hostile payload.
    // Response chỉ được nhắc package ta hỏi — ngoài đó là malformed/hostile.
    let lock = audit_lockfile_fixture();
    let payload = serde_json::json!({
        "some-random-package": [{
            "id": 1, "url": "u", "title": "t", "severity": "high",
            "vulnerable_versions": "*"
        }]
    });
    assert!(parse_advisory_bulk_response(&payload, &lock).is_err());
}

#[test]
fn test_audit_parse_rejects_malformed_advisory_entry() {
    // Missing npm Bulk API required fields (id/url/title/severity/
    // vulnerable_versions) or invalid ranges reject the entry and FAIL
    // the whole parse — no silent finding drop.
    // Thiếu field bắt buộc của npm Bulk API hoặc range sai thì từ chối
    // entry và FAIL cả parse — không bỏ finding âm thầm.
    let lock = audit_lockfile_fixture();
    let base = serde_json::json!({
        "id": 1102260,
        "url": "https://example.com/a",
        "title": "Prototype Pollution in lodash",
        "severity": "high",
        "vulnerable_versions": "<4.17.21"
    });
    let mut missing_id = base.clone();
    missing_id.as_object_mut().unwrap().remove("id");
    let mut missing_url = base.clone();
    missing_url.as_object_mut().unwrap().remove("url");
    let mut missing_title = base.clone();
    missing_title.as_object_mut().unwrap().remove("title");
    let mut missing_severity = base.clone();
    missing_severity.as_object_mut().unwrap().remove("severity");
    let mut missing_range = base.clone();
    missing_range
        .as_object_mut()
        .unwrap()
        .remove("vulnerable_versions");
    let bad_range = serde_json::json!({
        "id": 1102260, "url": "u", "title": "t", "severity": "high",
        "vulnerable_versions": "not a semver range !!!"
    });
    let not_array = serde_json::json!({"lodash": "oops"});
    let bad_entries = vec![
        missing_id,
        missing_url,
        missing_title,
        missing_severity,
        missing_range,
        bad_range,
        not_array,
    ];
    for payload in bad_entries {
        let full = if payload.get("lodash").is_some() || payload.as_str().is_some() {
            payload
        } else {
            serde_json::json!({"lodash": [payload]})
        };
        assert!(
            parse_advisory_bulk_response(&full, &lock).is_err(),
            "malformed payload must fail closed: {full}"
        );
    }
}

#[test]
fn test_audit_parse_ignores_findings_field_from_registry() {
    // The npm Bulk API never returns `findings`; if a future/hostile
    // registry includes one, MagiCore must IGNORE it and still match on
    // vulnerable_versions — findings are client-side truth only.
    // npm Bulk API không bao giờ trả `findings`; nếu registry lạ kèm
    // theo, MagiCore phải BỎ QUA và vẫn đối chiếu vulnerable_versions —
    // findings chỉ do client quyết định.
    let lock = audit_lockfile_fixture();
    let mut advisory = lodash_advisory_json();
    advisory["findings"] = serde_json::json!([{"version": "4.17.21", "paths": ["fake"]}]);
    let payload = serde_json::json!({"lodash": [advisory]});
    let vulns = parse_advisory_bulk_response(&payload, &lock).unwrap();
    // Still exactly the 4.17.12 entry (range match), NOT the injected 4.17.21.
    // Vẫn chỉ entry 4.17.12 (khớp range), KHÔNG phải 4.17.21 bị tiêm.
    assert_eq!(vulns.len(), 1);
    assert_eq!(vulns[0].package.version().to_string(), "4.17.12");
}

/// R3: gradle sidecar must ride the SHARED OSV-maven lane — a
/// verification-metadata.xml pin must reach a real query, never the old
/// "scanner not implemented" stub. Dead OSV endpoint keeps it hermetic:
/// the java step must EXIST (Failed naming osv-dev-api).
/// Sidecar gradle phải đi lane OSV-maven CHUNG — ghim phải tới query
/// thật, không còn stub. Endpoint chết giữ hermetic: step java phải TỒN
/// TẠI (Failed nêu osv-dev-api).
#[tokio::test]
async fn test_audit_gradle_sidecar_uses_shared_osv_maven_lane() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        r#"{"name":"w","version":"1.0.0"}"#,
    )
    .unwrap();
    std::fs::write(dir.path().join("build.gradle"), "// empty\n").unwrap();
    std::fs::create_dir_all(dir.path().join("gradle")).unwrap();
    std::fs::write(
        dir.path().join("gradle").join("verification-metadata.xml"),
        "<verification-metadata>\n<dependency group=\"com.example\" name=\"lib\" version=\"1.0\"/>\n</verification-metadata>\n",
    )
    .unwrap();

    let saved = std::env::var("MGC_OSV_API_BASE").ok();
    unsafe { std::env::set_var("MGC_OSV_API_BASE", "http://127.0.0.1:1/v1") };
    let adapter = WebAdapter::new().unwrap();
    let report = mgc_types::capabilities::AuditProvider::audit(&adapter, dir.path())
        .await
        .unwrap();
    match saved {
        Some(v) => unsafe { std::env::set_var("MGC_OSV_API_BASE", v) },
        None => unsafe { std::env::remove_var("MGC_OSV_API_BASE") },
    }
    match &report.scanner_status {
        mgc_types::adapter::ScannerStatus::Failed { scanner, .. } => {
            assert_eq!(scanner, "osv-dev-api")
        }
        other => panic!("gradle sidecar must reach the shared OSV lane (Failed), got {other:?}"),
    }
}

/// R3 dotnet side: a bare .csproj (no packages.lock.json) must surface
/// the shared lane's honest Unsupported (with remediation), never the
/// old "not implemented" stub and never a fake clean.
/// Csproj đơn lẻ phải ra Unsupported trung thực của lane chung (kèm
/// hướng dẫn), không stub cũ, không sạch giả.
#[tokio::test]
async fn test_audit_csproj_sidecar_unsupported_with_remediation() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        r#"{"name":"w","version":"1.0.0"}"#,
    )
    .unwrap();
    std::fs::write(dir.path().join("app.csproj"), "<Project/>\n").unwrap();

    let adapter = WebAdapter::new().unwrap();
    let report = mgc_types::capabilities::AuditProvider::audit(&adapter, dir.path())
        .await
        .unwrap();
    match &report.scanner_status {
        mgc_types::adapter::ScannerStatus::Partial { reasons, .. } => assert!(
            reasons.iter().any(|r| r.contains("packages.lock.json")),
            "dotnet lane must explain the missing lockfile, got: {reasons:?}"
        ),
        other => {
            panic!("bare csproj must aggregate Partial (unsupported dotnet lane), got {other:?}")
        }
    }
}
