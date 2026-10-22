// Focused tests for command-image priority and token positions.
// Test tập trung cho ưu tiên image và vị trí token.

use super::*;

#[test]
fn image_match_precedes_text_tokens_and_name_projection_is_stable() {
    assert_eq!(
        forbidden_process_match("npm.exe", "cargo install", &[]),
        Some(("npm".to_string(), "executable image", None, None))
    );
    assert_eq!(
        forbidden_process_match("node", "run npm install", &[]),
        Some((
            "npm".to_string(),
            "command-line token",
            Some("npm".to_string()),
            Some(1)
        ))
    );
    assert_eq!(
        forbidden_process_name("node", "npm install", &[]).as_deref(),
        Some("npm")
    );
}
