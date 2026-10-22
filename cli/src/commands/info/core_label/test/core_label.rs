// Existing package-classification behavior tests.
// Test hành vi phân loại package hiện tại.

use super::detect_core_label;

#[test]
fn classifies_each_existing_category() {
    let cases = [
        ("react", "", "[Core: Web / Frontend]"),
        ("express", "", "[Core: Web / Backend]"),
        ("openai", "", "[Core: AI / ML]"),
        ("terraform", "", "[Core: Cloud (clo)]"),
        ("github-actions", "", "[Core: CI/CD]"),
        ("bevy", "", "[Core: Game]"),
        ("flutter", "", "[Core: App (Mobile/Desktop)]"),
        ("serde", "serialization library", "[Core: General / Library]"),
    ];

    for (name, description, expected) in cases {
        assert_eq!(detect_core_label(name, description), expected, "{name}");
    }
}

#[test]
fn preserves_frontend_precedence_when_multiple_groups_match() {
    assert_eq!(
        detect_core_label("react-express", "backend server"),
        "[Core: Web / Frontend]"
    );
}
