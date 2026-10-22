use super::super::contains_embedded_sensitive_flag;

#[test]
fn detects_plain_quoted_escaped_and_continued_flags() {
    for command in [
        "npm install --token value",
        "npm install --'token' value",
        "npm install --token\\=value",
        "npm install --to\\\nken value",
        "npm install --to\\\r\nken value",
    ] {
        assert!(
            contains_embedded_sensitive_flag(command),
            "expected detection in {command:?}"
        );
    }
}

#[test]
fn ignores_command_without_sensitive_flag() {
    assert!(!contains_embedded_sensitive_flag(
        "npm install --path ./src"
    ));
}
