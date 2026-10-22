// Go scanner large-payload test.
// Kiểm thử payload lớn của scanner Go.

// Lane completion (P2 2026-09-10): reach the full 10-test/lane set.
// Hoàn tất lane (P2 2026-09-10): đủ bộ 10 test/lane.

#[test]
fn lane_go_7_large_stream_over_1mb_parses() {
    let total = 6_000;
    let mut stream = String::new();
    for i in 0..total {
        stream.push_str(&format!(
            r#"{{"osv":{{"id":"GO-2026-{i:05}","summary":"v{i}","database_specific":{{"severity":"LOW"}}}}}}"#
        ));
        stream.push('\n');
        stream.push_str(&format!(
            r#"{{"finding":{{"osv":"GO-2026-{i:05}","fixed_version":"v2.0.0","trace":[{{"module":"example.com/m{i:05}","version":"v1.0.0"}}]}}}}"#
        ));
        stream.push('\n');
    }
    assert!(stream.len() > 1_000_000, "fixture must exceed 1 MB");
    let parsed = parse_govulncheck_json(&stream).unwrap();
    assert_eq!(parsed.vulnerabilities.len(), total);
}
