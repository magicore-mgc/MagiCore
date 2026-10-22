// Bun lock parser tests — test parser lock Bun.
// Cover JSONC pins, scoped names, skipped identifiers, and fail-closed shapes.
// Bao phủ pin JSONC, tên có scope, identifier bị bỏ qua và shape fail-closed.

use super::read_bun_lock;
use mgc_types::MgError;

#[test]
fn parses_jsonc_scoped_pins_and_skipped_entries() {
    let raw = r#"{
      // Bun permits comments and trailing commas.
      // Bun cho phép comment và dấu phẩy cuối.
      "packages": {
        "@scope/pkg": ["@scope/pkg@2.1.0", "", {}, "sha512-x"],
        "workspace-root": ["workspace:root", "", {}, ""],
      },
    }"#;
    let read = read_bun_lock(raw).expect("parse Bun lock fixture");
    assert_eq!(read.npm_pins.len(), 1);
    assert_eq!(read.npm_pins[0].name, "@scope/pkg");
    assert_eq!(read.npm_pins[0].version, "2.1.0");
    assert_eq!(read.jsr_skipped[0].specifier, "workspace:root");
}

#[test]
fn rejects_non_array_entries_with_the_stable_error() {
    let error =
        read_bun_lock(r#"{"packages":{"pkg":"bad"}}"#).expect_err("reject non-array package entry");
    assert!(
        matches!(error, MgError::Other(message) if message == "bun.lock package 'pkg' entry must be an array")
    );
}

#[test]
fn rejects_non_string_first_entries_with_the_stable_error() {
    let error = read_bun_lock(r#"{"packages":{"pkg":[7]}}"#)
        .expect_err("reject non-string first package entry");
    assert!(
        matches!(error, MgError::Other(message) if message == "bun.lock package 'pkg' first entry must be a name@version string")
    );
}
