// Go stream malformed-input and reference validation tests.
// Kiểm thử dữ liệu sai định dạng và reference Go.

#[test]
fn dangling_osv_reference_fails_closed() {
    // A finding whose OSV entry NEVER appeared in the stream is a
    // contract violation — never a silent skip, never a fake clean.
    // Finding trỏ OSV chưa từng xuất hiện trong stream là vi phạm hợp
    // đồng — không bỏ âm thầm, không bịa sạch.
    let stream = concat!(
        r#"{"config":{"protocol_version":"v1.0.0"}}"#,
        "\n",
        r#"{"finding":{"osv":"GO-0000-0000","trace":[{"module":"example.com/x","version":"v1.0.0"}]}}"#,
        "\n",
    );
    assert!(parse_govulncheck_json(stream).is_err());
}

#[test]
fn malformed_line_fails_closed_not_clean() {
    // Truncated/garbage JSON line must ERROR — never report zero
    // findings from a stream we could not read.
    // Dòng JSON cắt quăng/rác phải LỖI — không báo 0 finding từ stream
    // đọc không nổi.
    for bad in ["", "not json", "{", r#"{"finding":"#] {
        assert!(
            parse_govulncheck_json(bad).is_err() || bad.is_empty(),
            "malformed stream must fail closed"
        );
    }
    // The empty string is special: zero messages is a valid (if odd)
    // empty stream — the SCANNER layer guards against it via exit codes.
    // Chuỗi rỗng là stream trống hợp lệ — tầng SCANNER chặn bằng exit
    // code.
}
