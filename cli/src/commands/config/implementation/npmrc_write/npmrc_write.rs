use super::super::{atomic_write_config_file, display_config_value, read_lines};
use crate::commands::config::implementation::read_optional_config_text;
use anyhow::Result;
use std::collections::BTreeMap;
use std::path::Path;

pub(in crate::commands::config) fn set_npmrc(path: &Path, key: &str, value: &str) -> Result<()> {
    let new_line = format!("{key}={value}");
    let lines = read_lines(path)?;
    let mut out: Vec<String> = Vec::new();
    let mut replaced = false;
    for line in lines {
        if line.starts_with(&format!("{key}=")) {
            out.push(new_line.clone());
            replaced = true;
        } else {
            out.push(line);
        }
    }
    if !replaced {
        out.push(new_line);
    }
    write_lines(path, &out)?;
    let shown = display_config_value(key, value);
    mgc_ui::success(&format!("set {key} = {shown}  →  {}", path.display()));
    Ok(())
}

pub(in crate::commands::config) fn delete_npmrc(path: &Path, key: &str) -> Result<()> {
    if !path.exists() {
        return Err(crate::error::config_key_missing(key));
    }
    let lines = read_lines(path)?;
    let out: Vec<String> = lines
        .into_iter()
        .filter(|line| !line.starts_with(&format!("{key}=")))
        .collect();
    write_lines(path, &out)?;
    mgc_ui::success(&format!("unset {key}  →  {}", path.display()));
    Ok(())
}

fn write_lines(path: &Path, lines: &[String]) -> Result<()> {
    let mut out = String::new();
    for line in lines {
        if !line.is_empty() {
            out.push_str(line);
        }
        out.push('\n');
    }
    atomic_write_config_file(path, out.as_bytes())
}

#[allow(dead_code)]
pub(in crate::commands::config::implementation) fn merge_file(
    merged: &mut BTreeMap<String, String>,
    path: &Path,
) {
    let Ok(Some(content)) = read_optional_config_text(path, ".npmrc") else {
        return;
    };
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }
        let Some((k, v)) = line.split_once('=') else {
            continue;
        };
        merged.insert(k.trim().to_string(), v.trim().to_string());
    }
}
