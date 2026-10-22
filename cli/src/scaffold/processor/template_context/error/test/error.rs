// Template-context typed-error tests.
// Kiểm tra lỗi typed template context.
#![allow(clippy::unwrap_used)]

use super::*;

#[test]
fn error_variants_preserve_existing_diagnostics_and_downcast_identity() {
    let cases = [
        (
            TemplateContextError::UndeclaredToken {
                token: "token".to_string(),
                template_path: "template.toml".to_string(),
            },
            "Template token 'token' in 'template.toml' is not declared in template.toml",
        ),
        (
            TemplateContextError::UnsupportedToken {
                token: "token".to_string(),
                template_path: "template.toml".to_string(),
            },
            "Template token 'token' in 'template.toml' is not supported by the Rust compiler context",
        ),
        (
            TemplateContextError::UnsupportedContext {
                key: "unknown".to_string(),
                template_path: "template.toml".to_string(),
            },
            concat!(
                "Template context 'unknown' required by 'template.toml' is not supported ",
                "by the Rust compiler context"
            ),
        ),
        (
            TemplateContextError::NoBoardSpecified {
                choices: "esp32c3".to_string(),
            },
            concat!(
                "No board specified — add `[iot] board = \"<id>\"` to mgc.toml ",
                "(known boards: esp32c3) or pass `mgc flash --board <id>`"
            ),
        ),
        (
            TemplateContextError::UnsupportedBoard {
                board: "unknown".to_string(),
                choices: "esp32c3".to_string(),
            },
            "Unsupported board 'unknown' — known boards: esp32c3",
        ),
        (
            TemplateContextError::IotFeatureUnavailable,
            "the iot core is not included in this MagiCore build",
        ),
    ];

    for (error, expected) in cases {
        let error: anyhow::Error = error.into();
        assert_eq!(error.to_string(), expected);
        assert!(error.downcast_ref::<TemplateContextError>().is_some());
    }
}
