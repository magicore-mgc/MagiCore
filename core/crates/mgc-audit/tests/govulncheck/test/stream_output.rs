// Go stream output and concurrency tests.
// Kiểm thử kết quả stream Go và chạy đồng thời.

#[test]
fn parses_vulnerable_stream_with_dedup_and_evidence() {
    let parsed = parse_govulncheck_json(VULNERABLE_STREAM).unwrap();
    // SBOM module count feeds packages_audited.
    // Số module trong SBOM là packages_audited.
    assert_eq!(parsed.packages_audited, 2);

    // GO-2021-0113 appears TWICE (module+package level) — deduped to one.
    // GO-2021-0113 xuất hiện HAI LẦN (level module+package) — gộp còn một.
    assert_eq!(parsed.vulnerabilities.len(), 2);

    let text_vuln = parsed
        .vulnerabilities
        .iter()
        .find(|v| v.cve == "CVE-2020-14040")
        .expect("CVE alias must surface");
    // Display name = LAST module path segment; full path in the title.
    // Tên hiển thị = ĐOẠN CUỐI path module; path đầy đủ trong title.
    assert_eq!(text_vuln.package.name().as_str(), "text");
    assert!(
        text_vuln.title.contains("golang.org/x/text"),
        "full module path must stay in the title: {}",
        text_vuln.title
    );
    assert_eq!(text_vuln.package.version().to_string(), "0.3.2");
    assert_eq!(text_vuln.patched_versions.as_deref(), Some("v0.3.3"));
    assert_eq!(text_vuln.scanner.as_deref(), Some("govulncheck"));
    assert_eq!(text_vuln.ecosystem.as_deref(), Some("go"));
    assert!(text_vuln.evidence_at.is_some(), "evidence must be stamped");
    assert!(
        text_vuln.url.as_deref().unwrap().contains("go.dev/issue"),
        "OSV reference URL must ride the finding"
    );
}

#[test]
fn clean_stream_is_zero_findings_with_module_count() {
    let parsed = parse_govulncheck_json(CLEAN_STREAM).unwrap();
    assert_eq!(parsed.packages_audited, 1);
    assert!(parsed.vulnerabilities.is_empty());
}

#[test]
fn pretty_printed_stream_parses_like_line_stream() {
    // govulncheck v1.8 emits PRETTY multi-line JSON objects — the parser
    // must handle both encodings identically (StreamDeserializer).
    // govulncheck v1.8 phát object JSON PRETTY nhiều dòng — parser phải
    // xử lý cả hai kiểu như nhau (StreamDeserializer).
    let pretty = r#"{
  "config": {
    "protocol_version": "v1.0.0"
  }
}
{
  "SBOM": {
    "modules": [
      {
        "path": "example.com/p",
        "version": "v1.0.0"
      }
    ]
  }
}
{
  "osv": {
    "id": "GO-2025-0001",
    "summary": "pretty stream vuln",
    "database_specific": {
      "severity": "MODERATE"
    }
  }
}
{
  "finding": {
    "osv": "GO-2025-0001",
    "fixed_version": "v1.0.1",
    "trace": [
      {
        "module": "example.com/p",
        "version": "v1.0.0"
      }
    ]
  }
}"#;
    let parsed = parse_govulncheck_json(pretty).unwrap();
    assert_eq!(parsed.packages_audited, 1);
    assert_eq!(parsed.vulnerabilities.len(), 1);
    assert!(matches!(
        parsed.vulnerabilities[0].severity_level,
        mgc_types::adapter::VulnerabilitySeverity::Medium
    ));
}

#[test]
fn pseudo_version_module_keeps_finding_with_real_version_in_title() {
    // golang.org/x/crypto pseudo-version v0.0.0-20211202 is NOT plain
    // semver — the finding MUST survive (0.0.0 carrier + real version in
    // title). Dropping it would hide a real advisory.
    // Pseudo-version v0.0.0-20211202 không phải semver thuần — finding
    // PHẢI sống sót (carrier 0.0.0 + version thật trong title). Bỏ nó
    // là giấu advisory thật.
    let parsed = parse_govulncheck_json(VULNERABLE_STREAM).unwrap();
    let crypto = parsed
        .vulnerabilities
        .iter()
        .find(|v| v.cve == "CVE-2021-43565")
        .unwrap();
    assert!(
        crypto.title.contains("v0.0.0-20211202"),
        "pseudo-version must be recorded in the title: {}",
        crypto.title
    );
}

#[test]
fn module_path_unrepresentable_falls_back_honestly() {
    // A module path that PackageName cannot hold must NEVER drop the
    // finding — the scanner keeps it with the tail segment as the
    // display name and the full path recorded in the title.
    // Module path mà PackageName không giữ được KHÔNG BAO GIỜ được bỏ
    // finding — scanner giữ với đoạn cuối làm tên hiển thị, path đầy
    // đủ ghi trong title.
    let stream = concat!(
        r#"{"osv":{"id":"GO-2024-0001","summary":"bad path module"}}"#,
        "\n",
        r#"{"finding":{"osv":"GO-2024-0001","trace":[{"module":"","version":"v1.0.0"}]}}"#,
        "\n",
    );
    let parsed = parse_govulncheck_json(stream).unwrap();
    assert_eq!(parsed.vulnerabilities.len(), 1);
    assert_eq!(
        parsed.vulnerabilities[0].package.name().as_str(),
        "unknown-module"
    );
}

#[test]
fn lane_go_8_empty_stream_is_zero_packages_not_fake_count() {
    // No SBOM message → packages_audited stays 0 (honest), findings 0.
    // Không message SBOM → packages_audited giữ 0 (trung thực).
    let parsed = parse_govulncheck_json("").unwrap();
    assert_eq!(parsed.packages_audited, 0);
    assert!(parsed.vulnerabilities.is_empty());
}

#[test]
fn lane_go_9_unicode_summary_and_module_survive() {
    let stream = concat!(
        r#"{"osv":{"id":"GO-2026-0009","summary":"lỗ hổng 🦀 unicode émoji"}}"#,
        "\n",
        r#"{"finding":{"osv":"GO-2026-0009","trace":[{"module":"example.com/m","version":"v1.0.0"}]}}"#,
        "\n",
    );
    let parsed = parse_govulncheck_json(stream).unwrap();
    assert!(parsed.vulnerabilities[0].title.contains("🦀"));
}

#[test]
fn lane_go_10_concurrent_streams_stay_independent() {
    let streams = [
        VULNERABLE_STREAM,
        CLEAN_STREAM,
        VULNERABLE_STREAM,
        CLEAN_STREAM,
    ];
    std::thread::scope(|scope| {
        let handles: Vec<_> = streams
            .iter()
            .map(|s| {
                let payload = s.to_string();
                scope.spawn(move || parse_govulncheck_json(&payload).unwrap())
            })
            .collect();
        for (i, h) in handles.into_iter().enumerate() {
            let parsed = h.join().unwrap();
            let expected = if i % 2 == 0 { 2 } else { 0 };
            assert_eq!(
                parsed.vulnerabilities.len(),
                expected,
                "stream {i} cross-contaminated"
            );
        }
    });
}
