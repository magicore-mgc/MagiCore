//! The coordinator preserves Bun JSONC and the public normalization contract.
//! Coordinator giữ JSONC Bun và hợp đồng chuẩn hóa public.

use super::strip_jsonc;

#[test]
fn bun_writer_shape_roundtrip() {
    // This fixture mirrors Bun's comments and trailing-comma output.
    // Fixture mô phỏng comment và dấu phẩy cuối do Bun ghi.
    let raw = concat!(
        "{\n  // this is a comment\n  \"packages\": {\n    \"left-pad\": [\n",
        "      \"left-pad@4.0.0\",\n",
        "      \"https://registry.npmjs.org/left-pad/-/left-pad-4.0.0.tgz\",\n",
        "      \"sha512-abc123\",\n    ],\n  },\n}\n",
    );
    let output = strip_jsonc(raw);
    let parsed: serde_json::Value = serde_json::from_str(&output)
        .unwrap_or_else(|error| panic!("Bun JSONC must parse after cleanup ({error}): {output}"));
    let entry = &parsed["packages"]["left-pad"];

    assert_eq!(entry[0].as_str(), Some("left-pad@4.0.0"));
    assert!(matches!(entry[1].as_str(), Some(url) if url.starts_with("https://")));
    assert!(matches!(entry[2].as_str(), Some(hash) if hash.starts_with("sha512-")));
}

#[test]
fn empty_and_blank_input_yield_no_content() {
    // Empty input stays empty; blank lines retain their content.
    // Input rỗng vẫn rỗng; dòng trắng giữ nguyên nội dung.
    assert_eq!(strip_jsonc(""), "");
    assert_eq!(strip_jsonc("   \n\n"), "   \n\n");
}

#[test]
fn crlf_input_keeps_the_existing_lf_normalization() {
    // The historical API emits LF lines even for CRLF input.
    // API cũ xuất dòng LF kể cả khi input dùng CRLF.
    let input = "{\r\n  \"ok\": true\r\n}\r\n";
    let expected = "{\n  \"ok\": true\n}\n";

    assert_eq!(strip_jsonc(input), expected);
}
