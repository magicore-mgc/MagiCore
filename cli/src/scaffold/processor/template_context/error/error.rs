// Typed errors for template token contracts and Core target selection.
// Lỗi typed cho contract token và chọn target Core.

#[derive(Debug, thiserror::Error)]
pub(in crate::scaffold::processor::template_context) enum TemplateContextError {
    #[error("Template token '{token}' in '{template_path}' is not declared in template.toml")]
    UndeclaredToken {
        token: String,
        template_path: String,
    },
    #[error(
        "Template token '{token}' in '{template_path}' is not supported by the Rust compiler context"
    )]
    UnsupportedToken {
        token: String,
        template_path: String,
    },
    #[error(
        "Template context '{key}' required by '{template_path}' is not supported by the Rust compiler context"
    )]
    UnsupportedContext { key: String, template_path: String },
    #[error(
        "No board specified — add `[iot] board = \"<id>\"` to mgc.toml \
         (known boards: {choices}) or pass `mgc flash --board <id>`"
    )]
    NoBoardSpecified { choices: String },
    #[error("Unsupported board '{board}' — known boards: {choices}")]
    UnsupportedBoard { board: String, choices: String },
    #[cfg_attr(feature = "iot", allow(dead_code))]
    #[error("the iot core is not included in this MagiCore build")]
    IotFeatureUnavailable,
}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
