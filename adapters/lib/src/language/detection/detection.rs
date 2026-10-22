//! Lib language detection precedence.
//! Thứ tự ưu tiên nhận diện ngôn ngữ Lib.

use std::path::Path;

use crate::language::types::LibLanguage;

use super::project_config::ProjectLanguageOutcome;

/// Detect the configured language or the first matching marker.
/// Nhận diện ngôn ngữ config hoặc marker đầu tiên khớp.
pub fn detect_language(root: &Path) -> Option<LibLanguage> {
    match super::project_config::read_project_language(root) {
        ProjectLanguageOutcome::Configured(language) => return Some(language),
        ProjectLanguageOutcome::IgnoredJsProject | ProjectLanguageOutcome::UnsupportedLanguage => {
            return None;
        }
        ProjectLanguageOutcome::NotConfigured => {}
    }
    super::markers::detect_marker_language(root)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
