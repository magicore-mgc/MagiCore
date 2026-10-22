//! Reader and policy for project language configuration.
//! Reader và policy config ngôn ngữ project.

use std::path::Path;

use crate::language::{LibLanguage, is_regular_manifest};

use super::super::language_name::language_from_str;

/// Preserve the three existing project-config short-circuit states.
/// Giữ ba trạng thái short-circuit hiện hành của project config.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::language::detection) enum ProjectLanguageOutcome {
    NotConfigured,
    Configured(LibLanguage),
    UnsupportedLanguage,
    IgnoredJsProject,
}

/// Read the language while preserving the existing ecosystem exclusion.
/// Đọc ngôn ngữ và giữ điều kiện loại trừ ecosystem hiện hành.
pub(in crate::language::detection) fn read_project_language(root: &Path) -> ProjectLanguageOutcome {
    let Ok(Some(content)) =
        mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")
    else {
        return ProjectLanguageOutcome::NotConfigured;
    };
    let Ok(value) = toml::from_str::<toml::Value>(&content) else {
        return ProjectLanguageOutcome::NotConfigured;
    };

    if value
        .get("ecosystem")
        .and_then(toml::Value::as_str)
        .is_some_and(|ecosystem| ecosystem != "lib")
        && value.get("lib").is_none()
        && is_regular_manifest(&root.join("package.json"))
    {
        return ProjectLanguageOutcome::IgnoredJsProject;
    }

    let Some(name) = value
        .get("lib")
        .and_then(|section| section.get("language"))
        .and_then(toml::Value::as_str)
    else {
        return ProjectLanguageOutcome::NotConfigured;
    };

    match language_from_str(name) {
        Some(language) => ProjectLanguageOutcome::Configured(language),
        None => ProjectLanguageOutcome::UnsupportedLanguage,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
