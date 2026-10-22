//! Cargo TOML root validation and section dispatch.
//! (Kiểm tra root Cargo TOML và điều phối các section.)

use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

/// Parse only the Cargo settings that the optimizer deliberately supports.
/// Chỉ parse các setting Cargo mà optimizer chủ đích hỗ trợ.
pub(in crate::commands::optimizer::env_loader) fn parse_toml_file(
    path: &Path,
) -> Result<HashMap<String, String>> {
    let content = crate::commands::optimizer::env_loader::project_files::read_optimizer_text(path)?;
    let toml: toml::Value = toml::from_str(&content).map_err(|error| {
        crate::commands::optimizer::env_loader::error::setting_invalid(path, "TOML").context(error)
    })?;

    let Some(root) = toml.as_table() else {
        return Err(crate::commands::optimizer::env_loader::error::setting_invalid(path, "root"));
    };

    if root
        .keys()
        .any(|key| !matches!(key.as_str(), "build" | "profile" | "env"))
    {
        return Err(
            crate::commands::optimizer::env_loader::error::setting_invalid(path, "top-level key"),
        );
    }

    let mut vars = HashMap::new();
    if let Some(env_table) = root.get("env") {
        vars.extend(super::env_settings::parse_env_settings(path, env_table)?);
    }
    if let Some(build_table) = root.get("build") {
        vars.extend(super::build_settings::parse_build_settings(
            path,
            build_table,
        )?);
    }
    if let Some(profile_table) = root.get("profile") {
        vars.extend(super::profile_settings::parse_profile_settings(
            path,
            profile_table,
        )?);
    }

    Ok(vars)
}
