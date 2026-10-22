//! Pin each existing search classification group and its precedence.
//! Cố định từng nhóm phân loại search hiện có và độ ưu tiên của chúng.

use super::super::detect_core_label;

#[test]
fn classifies_current_search_keyword_groups_and_default() {
    let cases = [
        ("react", "", "[Core: Web / Frontend]"),
        ("express", "", "[Core: Web / Backend]"),
        ("openai", "", "[Core: AI / ML]"),
        ("terraform", "", "[Core: Cloud (clo)]"),
        ("github-actions", "", "[Core: CI/CD]"),
        ("bevy", "", "[Core: Game]"),
        ("flutter", "", "[Core: App (Mobile/Desktop)]"),
        ("serde", "serialization library", "[Core: Web / Node]"),
    ];

    for (name, description, expected) in cases {
        assert_eq!(detect_core_label(name, description), expected, "{name}");
    }
}

#[test]
fn keeps_frontend_precedence_when_multiple_groups_match() {
    assert_eq!(
        detect_core_label("react-express", "backend server"),
        "[Core: Web / Frontend]"
    );
}
