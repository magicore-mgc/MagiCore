use super::super::{display_config_value, file_value, find_mgc_toml, toml_value};
use anyhow::Result;

pub(in crate::commands::config) fn get(key: &str) -> Result<()> {
    // 1. Environment variable
    if let Some(value) = env_value(key) {
        let shown = display_config_value(key, &value);
        mgc_ui::info(&format!("[env] {key} = {shown}"));
        println!("{shown}");
        return Ok(());
    }
    // 2. mgc.toml project
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Some(toml_path) = find_mgc_toml()
        && let Some(value) = toml_value(&toml_path, key)
    {
        let shown = display_config_value(key, &value);
        mgc_ui::info(&format!("[mgc.toml] {key} = {shown}"));
        println!("{shown}");
        return Ok(());
    }
    // 3. .npmrc local (project CWD)
    let project_npmrc = std::env::current_dir()?.join(".npmrc");
    if let Some(value) = file_value(&project_npmrc, key)? {
        let shown = display_config_value(key, &value);
        mgc_ui::info(&format!("[.npmrc local] {key} = {shown}"));
        println!("{shown}");
        return Ok(());
    }
    // 4. .npmrc user (~/)
    if let Some(home) = dirs::home_dir() {
        let user_npmrc = home.join(".npmrc");
        if let Some(value) = file_value(&user_npmrc, key)? {
            let shown = display_config_value(key, &value);
            mgc_ui::info(&format!("[.npmrc user] {key} = {shown}"));
            println!("{shown}");
            return Ok(());
        }
    }
    Err(crate::error::config_key_missing(key))
}

fn env_value(key: &str) -> Option<String> {
    let normalized = key.to_uppercase().replace('-', "_");
    for candidate in [
        format!("MGC_{normalized}"),
        format!("npm_config_{normalized}"),
    ] {
        if let Ok(value) = std::env::var(&candidate) {
            return Some(value);
        }
    }
    None
}
