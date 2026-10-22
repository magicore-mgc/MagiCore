//! Encoding error display tests — Test display lỗi encoding.
//! Keep parser diagnostics stable at the private owner boundary — Giữ diagnostic parser ổn định tại owner private.

use super::super::EncodingError;

#[test]
fn local_parse_errors_keep_their_display_text() {
    let errors = [
        (
            EncodingError::InvalidHex("odd length".to_string()),
            "odd length",
        ),
        (
            EncodingError::InvalidBase64("invalid symbol".to_string()),
            "invalid symbol",
        ),
        (
            EncodingError::InvalidDigestLength(31),
            "expected 32 bytes, got 31",
        ),
    ];

    for (error, expected) in errors {
        assert_eq!(error.to_string(), expected);
    }
}
