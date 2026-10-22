//! Typed local errors for scaffold specification parsing and validation.
//! Lỗi typed cục bộ cho parse và validation scaffold specification.

/// A validation/parser failure with the original user-facing message.
/// Lỗi validation/parser, giữ nguyên thông báo hiện có cho người dùng.
#[derive(Debug, thiserror::Error)]
pub(in crate::scaffold::spec) enum ScaffoldSpecError {
    #[error("Template name cannot be empty")]
    TemplateNameEmpty,
    #[error("Empty dist tag")]
    EmptyDistTag,
    #[error("Unknown tag '{tag}'. Did you mean '{suggestion}'?")]
    UnknownTagWithSuggestion { tag: String, suggestion: String },
    #[error("Empty version")]
    EmptyVersion,
    #[error("Version must start with digit, got '{version}'")]
    VersionMustStartWithDigit { version: String },
    #[error("Version ranges not supported yet")]
    VersionRangesUnsupported,
}

impl ScaffoldSpecError {
    /// Convert into the same message-only `anyhow` form used before extraction.
    /// Chuyển thành dạng message-only `anyhow` như trước khi tách module.
    pub(in crate::scaffold::spec) fn into_anyhow(self) -> anyhow::Error {
        anyhow::Error::msg(self.to_string())
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
