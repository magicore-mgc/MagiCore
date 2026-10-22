//! Parse and validate the supported Cargo profile tables.
//! (Parse và kiểm tra các bảng Cargo profile được hỗ trợ.)

use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

pub(in crate::commands::optimizer::env_loader::cargo_toml) fn parse_profile_settings(
    path: &Path,
    value: &toml::Value,
) -> Result<HashMap<String, String>> {
    let Some(profiles) = value.as_table() else {
        return Err(
            crate::commands::optimizer::env_loader::error::setting_invalid(path, "profile"),
        );
    };
    let mut vars = HashMap::new();
    for (profile, settings) in profiles {
        if !matches!(profile.as_str(), "dev" | "release" | "test" | "bench") {
            return Err(
                crate::commands::optimizer::env_loader::error::setting_invalid(
                    path,
                    "profile name",
                ),
            );
        }
        let Some(settings) = settings.as_table() else {
            return Err(
                crate::commands::optimizer::env_loader::error::setting_invalid(
                    path,
                    "profile table",
                ),
            );
        };
        for (setting, value) in settings {
            let env_key = cargo_profile_env_key(profile, setting);
            let value = validate_profile_value(path, setting, value)?;
            vars.insert(env_key, value);
        }
    }
    Ok(vars)
}

/// Convert a Cargo profile field into its supported environment key.
/// Chuyển field Cargo profile thành key môi trường được hỗ trợ.
fn cargo_profile_env_key(profile: &str, setting: &str) -> String {
    format!(
        "CARGO_PROFILE_{}_{}",
        profile.to_ascii_uppercase(),
        setting.replace('-', "_").to_ascii_uppercase()
    )
}

/// Validate the intentionally small profile surface before passing it to Cargo.
/// Kiểm tra whitelist profile nhỏ trước khi chuyển cấu hình cho Cargo.
fn validate_profile_value(path: &Path, setting: &str, value: &toml::Value) -> Result<String> {
    let valid = match setting {
        "opt-level" => {
            value
                .as_integer()
                .is_some_and(|level| (0..=3).contains(&level))
                || value
                    .as_str()
                    .is_some_and(|level| matches!(level, "s" | "z"))
        }
        "codegen-units" => value
            .as_integer()
            .is_some_and(|units| (1..=65_536).contains(&units)),
        "debug" => {
            value.as_bool().is_some()
                || value
                    .as_integer()
                    .is_some_and(|level| (0..=2).contains(&level))
                || value.as_str().is_some_and(|level| {
                    matches!(
                        level,
                        "none" | "limited" | "line-tables-only" | "line-directives-only" | "full"
                    )
                })
        }
        "lto" => {
            value.as_bool().is_some()
                || value
                    .as_str()
                    .is_some_and(|mode| matches!(mode, "thin" | "fat" | "off"))
        }
        "panic" => value
            .as_str()
            .is_some_and(|mode| matches!(mode, "abort" | "unwind")),
        _ => {
            return Err(
                crate::commands::optimizer::env_loader::error::setting_invalid(path, "profile key"),
            );
        }
    };
    if !valid {
        return Err(crate::commands::optimizer::env_loader::error::setting_invalid(path, setting));
    }
    Ok(value.to_string())
}
