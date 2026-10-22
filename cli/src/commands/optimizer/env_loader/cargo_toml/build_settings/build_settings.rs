//! Validate Cargo TOML build jobs and rustflags.
//! (Kiểm tra jobs và rustflags trong Cargo TOML.)

use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

pub(in crate::commands::optimizer::env_loader::cargo_toml) fn parse_build_settings(
    path: &Path,
    value: &toml::Value,
) -> Result<HashMap<String, String>> {
    let Some(build) = value.as_table() else {
        return Err(crate::commands::optimizer::env_loader::error::setting_invalid(path, "build"));
    };
    if build
        .keys()
        .any(|key| !matches!(key.as_str(), "jobs" | "rustflags"))
    {
        return Err(
            crate::commands::optimizer::env_loader::error::setting_invalid(path, "build key"),
        );
    }
    let mut vars = HashMap::new();
    if let Some(jobs) = build.get("jobs") {
        let Some(jobs) = jobs.as_integer().filter(|jobs| *jobs > 0) else {
            return Err(
                crate::commands::optimizer::env_loader::error::setting_invalid(path, "build.jobs"),
            );
        };
        vars.insert("CARGO_BUILD_JOBS".to_string(), jobs.to_string());
    }
    if let Some(rustflags) = build.get("rustflags") {
        let Some(rustflags) = rustflags.as_array() else {
            return Err(
                crate::commands::optimizer::env_loader::error::setting_invalid(
                    path,
                    "build.rustflags",
                ),
            );
        };
        let rustflags = rustflags
            .iter()
            .map(|value| {
                value.as_str().map(str::to_string).ok_or_else(|| {
                    crate::commands::optimizer::env_loader::error::setting_invalid(
                        path,
                        "build.rustflags item",
                    )
                })
            })
            .collect::<Result<Vec<_>>>()?;
        if !rustflags.is_empty() {
            vars.insert("RUSTFLAGS".to_string(), rustflags.join(" "));
        }
    }
    Ok(vars)
}
