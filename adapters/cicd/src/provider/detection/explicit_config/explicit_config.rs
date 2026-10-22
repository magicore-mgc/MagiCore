//! Reader for the explicit provider in project mgc.toml.
//! Reader provider tường minh trong mgc.toml của project.

use std::path::Path;

use crate::provider::CicdProvider;

use super::super::provider_name::provider_from_str;

/// Preserve whether a valid string names an unsupported provider.
/// Giữ riêng trường hợp chuỗi hợp lệ về kiểu nhưng tên provider chưa hỗ trợ.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::provider::detection) enum ProviderConfigOutcome {
    NotConfigured,
    Recognized(CicdProvider),
    UnsupportedName,
}

/// Read the project provider while retaining the old fallback distinction.
/// Đọc provider project và giữ khác biệt fallback hiện hành.
pub(in crate::provider::detection) fn read_configured_provider(
    root: &Path,
) -> ProviderConfigOutcome {
    let Ok(Some(content)) =
        mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")
    else {
        return ProviderConfigOutcome::NotConfigured;
    };
    let Ok(value) = toml::from_str::<toml::Value>(&content) else {
        return ProviderConfigOutcome::NotConfigured;
    };
    let Some(name) = value
        .get("cicd")
        .and_then(|section| section.get("provider"))
        .and_then(toml::Value::as_str)
    else {
        return ProviderConfigOutcome::NotConfigured;
    };

    match provider_from_str(name) {
        Some(provider) => ProviderConfigOutcome::Recognized(provider),
        None => ProviderConfigOutcome::UnsupportedName,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
