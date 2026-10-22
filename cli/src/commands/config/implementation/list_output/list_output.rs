use super::super::{find_mgc_toml, is_sensitive, read_optional_config_text, redact_toml_config};
use anyhow::Result;
use std::path::Path;

pub(in crate::commands::config) fn list(local_only: bool) -> Result<()> {
    let mut any = false;

    // 1. MGC_* env vars
    if !local_only {
        let env_keys: Vec<(String, String)> = std::env::vars()
            .filter(|(k, _)| k.starts_with("MGC_") || k.starts_with("npm_config_"))
            .collect();
        if !env_keys.is_empty() {
            println!("# [env]");
            for (k, v) in &env_keys {
                let shown = if is_sensitive(k) { "***" } else { v.as_str() };
                println!("  {k} = {shown}");
            }
            any = true;
        }
    }

    // 2. mgc.toml
    if let Some(toml_path) = find_mgc_toml() {
        let content = mgc_adapter_base::project_file::read_regular_text(&toml_path, "mgc.toml")?;
        if !content.trim().is_empty() {
            println!("# [mgc.toml] {}", toml_path.display());
            // In dạng flat (bỏ comment, chỉ key = value)
            for line in redact_toml_config(&content)?.lines() {
                println!("  {line}");
            }
            any = true;
        }
    }

    // 3. .npmrc project
    let project_npmrc = std::env::current_dir()?.join(".npmrc");
    if read_optional_config_text(&project_npmrc, ".npmrc")?.is_some() {
        println!("# [.npmrc local] {}", project_npmrc.display());
        print_npmrc_file(&project_npmrc)?;
        any = true;
    }

    // 4. .npmrc user
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if !local_only && let Some(home) = dirs::home_dir() {
        let user_npmrc = home.join(".npmrc");
        if read_optional_config_text(&user_npmrc, ".npmrc")?.is_some() {
            println!("# [.npmrc user] {}", user_npmrc.display());
            print_npmrc_file(&user_npmrc)?;
            any = true;
        }
    }

    if !any {
        mgc_ui::info("no configuration found (.npmrc / mgc.toml)");
    }
    Ok(())
}

/// Return local configuration with secrets redacted and without printing it.
/// Trả cấu hình local đã che bí mật mà không in ra stdout.
pub fn list_local_redacted() -> Result<String> {
    let mut output = String::new();
    if let Some(toml_path) = find_mgc_toml() {
        let content = mgc_adapter_base::project_file::read_regular_text(&toml_path, "mgc.toml")?;
        if !content.trim().is_empty() {
            output.push_str("# [mgc.toml] ");
            output.push_str(&toml_path.display().to_string());
            output.push('\n');
            for line in redact_toml_config(&content)?.lines() {
                output.push_str("  ");
                output.push_str(line);
                output.push('\n');
            }
        }
    }

    let project_npmrc = std::env::current_dir()?.join(".npmrc");
    if let Some(content) = read_optional_config_text(&project_npmrc, ".npmrc")? {
        output.push_str("# [.npmrc local] ");
        output.push_str(&project_npmrc.display().to_string());
        output.push('\n');
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                output.push_str("  ");
                output.push_str(key.trim());
                output.push_str(" = ");
                output.push_str(if is_sensitive(key) {
                    "***"
                } else {
                    value.trim()
                });
                output.push('\n');
            }
        }
    }

    if output.is_empty() {
        output.push_str("no configuration found (.npmrc / mgc.toml)");
    }
    Ok(output)
}

fn print_npmrc_file(path: &Path) -> Result<()> {
    let Some(content) = read_optional_config_text(path, ".npmrc")? else {
        return Ok(());
    };
    for line in content.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') || t.starts_with(';') {
            continue;
        }
        if let Some((k, v)) = t.split_once('=') {
            let shown = if is_sensitive(k) { "***" } else { v.trim() };
            println!("  {} = {}", k.trim(), shown);
        }
    }
    Ok(())
}
