//! Trailing-comma cleanup preserves legal JSON separators.
//! Dọn dấu phẩy cuối nhưng giữ dấu phân cách JSON hợp lệ.

use super::strip_trailing_commas;

#[test]
fn trailing_comma_before_close_bracket_is_stripped() {
    // Bun's pretty output places the closing token on the next line.
    // Output pretty của Bun đặt token đóng ở dòng tiếp theo.
    let raw = "{\n  \"a\": 1,\n  \"b\": [\n    \"x\",\n    \"y\",\n  ]\n}\n";
    let output = strip_trailing_commas(raw);
    let parsed: serde_json::Value = serde_json::from_str(&output)
        .unwrap_or_else(|error| panic!("multiline trailing commas must parse ({error}): {output}"));

    assert_eq!(parsed["b"].as_array().map(|items| items.len()), Some(2));
}

#[test]
fn comma_before_next_member_is_kept() {
    let raw = "{\n  \"a\": 1,\n  \"b\": 2,\n  \"c\": 3\n}\n";
    let output = strip_trailing_commas(raw);
    let parsed: serde_json::Value = serde_json::from_str(&output)
        .unwrap_or_else(|error| panic!("legal commas must survive ({error}): {output}"));

    assert_eq!(parsed["c"].as_i64(), Some(3));
}

#[test]
fn inline_trailing_commas_are_removed_without_changing_string_values() {
    let raw = r#"{"items":[1,],"text":"value\",]tail",}"#;
    let output = strip_trailing_commas(raw);
    let parsed: serde_json::Value = serde_json::from_str(&output)
        .unwrap_or_else(|error| panic!("inline trailing commas must parse ({error}): {output}"));

    assert_eq!(parsed["items"][0].as_i64(), Some(1));
    assert_eq!(parsed["text"].as_str(), Some("value\",]tail"));
}
