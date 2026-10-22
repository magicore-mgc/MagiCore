//! Validate Cargo TOML `[env]` values and reserved keys.
//! (Kiểm tra giá trị `[env]` trong Cargo TOML và key bị dành riêng.)

use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

pub(in crate::commands::optimizer::env_loader::cargo_toml) fn parse_env_settings(
    path: &Path,
    value: &toml::Value,
) -> Result<HashMap<String, String>> {
    let Some(env_table) = value.as_table() else {
        return Err(crate::commands::optimizer::env_loader::error::setting_invalid(path, "env"));
    };
    let mut vars = HashMap::new();
    for (key, value) in env_table {
        let Some(value) = value.as_str() else {
            return Err(
                crate::commands::optimizer::env_loader::error::setting_invalid(path, "env value"),
            );
        };
        if reserved_optimizer_env_key(key) {
            return Err(
                crate::commands::optimizer::env_loader::error::setting_invalid(
                    path,
                    "reserved env key",
                ),
            );
        }
        vars.insert(key.clone(), value.to_string());
    }
    Ok(vars)
}

/// Cargo-owned process and loader settings cannot be overridden from `[env]`.
/// Không cho `[env]` ghi đè biến điều khiển Cargo hoặc nạp mã động.
fn reserved_optimizer_env_key(key: &str) -> bool {
    let key = key.to_ascii_uppercase();
    matches!(
        key.as_str(),
        "PATH"
            | "HOME"
            | "USERPROFILE"
            | "CARGO_HOME"
            | "CARGO_TARGET_DIR"
            | "RUSTC"
            | "RUSTC_WRAPPER"
            | "RUSTC_WORKSPACE_WRAPPER"
            | "RUSTFLAGS"
            | "CARGO_ENCODED_RUSTFLAGS"
            | "NODE_OPTIONS"
            | "PYTHONPATH"
            | "PYTHONHOME"
            | "LD_PRELOAD"
            | "DYLD_INSERT_LIBRARIES"
    ) || key.starts_with("CARGO_BUILD_")
        || key.starts_with("CARGO_PROFILE_")
        || key.starts_with("LD_")
        || key.starts_with("DYLD_")
}
