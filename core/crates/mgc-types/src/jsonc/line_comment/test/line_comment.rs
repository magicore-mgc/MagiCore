//! Comment scanning preserves JSON strings and removes outside comments.
//! Quét comment giữ JSON string và bỏ comment bên ngoài.

use super::strip_line_comments;

#[test]
fn line_comments_outside_strings_are_stripped() {
    let raw = "{\n  // registry comment\n  \"name\": \"left\",\n}\n";
    let output = strip_line_comments(raw);

    assert!(!output.contains("registry comment"));
    assert!(output.contains("\"left\""));
}

#[test]
fn slash_slash_inside_string_is_not_a_comment() {
    // A tarball URL contains `//`; string contents must survive.
    // URL tarball chứa `//`; nội dung string phải được giữ nguyên.
    let raw = "{\n  \"resolved\": \"https://registry.npmjs.org/pkg/-/pkg-1.0.0.tgz\",\n}\n";
    let output = strip_line_comments(raw);

    assert!(output.contains("https://registry.npmjs.org/pkg/-/pkg-1.0.0.tgz"));
}

#[test]
fn escaped_quote_does_not_expose_string_slashes_as_comment() {
    let raw = r#"{"resolved":"https://registry.example/pkg/\"//kept","name":"sample"} // outside"#;
    let output = strip_line_comments(raw);
    let parsed: serde_json::Value = serde_json::from_str(&output)
        .unwrap_or_else(|error| panic!("escaped JSON string must survive ({error}): {output}"));

    assert_eq!(parsed["resolved"], "https://registry.example/pkg/\"//kept");
    assert_eq!(parsed["name"], "sample");
    assert!(!output.contains("outside"));
}
