// Deno lock parser tests — test parser lock Deno.
// Cover npm/JSR extraction and malformed lock/pin rejection.
// Bao phủ trích npm/JSR và từ chối lockfile hoặc pin sai.

use super::read_deno_lock;
use mgc_types::MgError;

#[test]
fn parses_npm_pins_and_records_jsr_entries() {
    let raw = r#"{"npm":{"lodash@4.17.20":{"integrity":"sha512-x"}},"jsr":{"@std/bytes@0.224.0":{"integrity":"x"}}}"#;
    let read = read_deno_lock(raw).expect("parse Deno lock fixture");
    assert_eq!(read.npm_pins.len(), 1);
    assert_eq!(read.npm_pins[0].name, "lodash");
    assert_eq!(read.npm_pins[0].version, "4.17.20");
    assert_eq!(read.jsr_skipped[0].specifier, "@std/bytes@0.224.0");
}

#[test]
fn rejects_versionless_npm_pins_with_the_stable_error() {
    let error = read_deno_lock(r#"{"npm":{"lodash":{}}}"#).expect_err("reject versionless npm pin");
    assert!(
        matches!(error, MgError::Other(message) if message == "deno.lock npm pin 'lodash' must be name@version")
    );
}
