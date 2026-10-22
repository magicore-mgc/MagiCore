#![allow(clippy::unwrap_used)]
//! Path encoding owner tests — Kiểm thử owner mã hóa đường dẫn

use super::super::*;

#[test]
fn package_path_encoding_preserves_separators_and_escapes_reserved_bytes() {
    assert_eq!(purl_path("@scope/core"), "%40scope/core");
    assert_eq!(percent_encode_path("1.0.0+meta"), "1.0.0%2Bmeta");
    assert_eq!(percent_decode_path_segment("org%2Flib"), "org/lib");
}
