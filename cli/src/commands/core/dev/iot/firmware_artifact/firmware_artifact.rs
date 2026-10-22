use anyhow::{Result, anyhow, bail};
use std::path::{Path, PathBuf};

/// Tìm binary ELF đã build — ưu tiên target/<triple>/release/<name>.elf, fallback scan.
pub(in crate::commands::core::dev) fn find_elf(root: &Path, target: &str) -> Result<PathBuf> {
    if !root.join("Cargo.toml").exists() {
        bail!("No Cargo.toml found — esp32-rust project missing");
    }
    let name: String = std::fs::read_to_string(root.join("Cargo.toml"))
        .ok()
        .and_then(|s| toml::from_str::<toml::Value>(&s).ok())
        .and_then(|v| {
            v.get("package")
                .and_then(|p| p.get("name"))
                .and_then(|n| n.as_str())
                .map(str::to_string)
        })
        .unwrap_or_else(|| "firmware".to_string());

    let target_dir = root.join("target");
    if !target_dir.exists() {
        bail!(
            "No target/ directory found — build the firmware first (or run without --skip-build)"
        );
    }
    let preferred = target_dir
        .join(target)
        .join("release")
        .join(format!("{name}.elf"));
    if preferred.exists() {
        return Ok(preferred);
    }
    let mut matches: Vec<PathBuf> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&target_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let release = path.join("release");
            let elf = release.join(format!("{name}.elf"));
            if elf.exists() {
                matches.push(elf);
            }
        }
    }
    if matches.is_empty() {
        bail!(
            "No {name}.elf found in target/*/release — build the firmware first (or run without --skip-build)"
        );
    }
    matches.sort();
    matches
        .pop()
        .ok_or_else(|| anyhow!("No firmware artifact was selected"))
}

#[cfg(test)]
#[path = "test/firmware_artifact.rs"]
mod tests;
