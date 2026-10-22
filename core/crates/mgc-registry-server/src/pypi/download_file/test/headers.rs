//! Content-Disposition filename safety tests.
//! Kiểm thử an toàn filename trong Content-Disposition.

use super::super::download_file::attachment_disposition;

#[test]
fn ordinary_filename_has_compatible_fallback_and_utf8_parameter() {
    let header = attachment_disposition("demo-1.0.0.whl").unwrap();
    assert_eq!(
        header.to_str().unwrap(),
        "attachment; filename=\"demo-1.0.0.whl\"; filename*=UTF-8''demo-1.0.0.whl"
    );
}

#[test]
fn quotes_and_backslashes_cannot_inject_parameters() {
    let header = attachment_disposition(r#"bad"; filename=evil\name.whl"#).unwrap();
    assert_eq!(
        header.to_str().unwrap(),
        "attachment; filename=\"bad__ filename_evil_name.whl\"; filename*=UTF-8''bad%22%3B%20filename%3Devil%5Cname.whl"
    );
}

#[test]
fn line_breaks_and_unicode_are_encoded_into_ascii_header_values() {
    let line_breaks = attachment_disposition("evil\r\nx.whl").unwrap();
    assert_eq!(
        line_breaks.to_str().unwrap(),
        "attachment; filename=\"evil__x.whl\"; filename*=UTF-8''evil%0D%0Ax.whl"
    );
    let unicode = attachment_disposition("café.whl").unwrap();
    assert_eq!(
        unicode.to_str().unwrap(),
        "attachment; filename=\"caf_.whl\"; filename*=UTF-8''caf%C3%A9.whl"
    );
}
