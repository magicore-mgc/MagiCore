//! Public integration contract for the crate-root JSONC export.
//! Contract integration cho export JSONC tại crate root.

use mgc_types::strip_jsonc;

#[test]
fn public_export_accepts_inline_bun_jsonc() {
    let output = strip_jsonc(r#"{"packages":["pkg@1.0.0",],}"#);
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap_or_else(|error| {
        panic!("public JSONC export must produce JSON ({error}): {output}")
    });

    assert_eq!(parsed["packages"][0].as_str(), Some("pkg@1.0.0"));
}
