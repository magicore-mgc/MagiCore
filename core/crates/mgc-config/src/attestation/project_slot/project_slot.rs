//! External project identity slots / Identity slot bên ngoài project.

use std::path::{Path, PathBuf};

use crate::attestation::blake3_hex;

/// Attestation file name inside a project slot — Tên file chứng thực.
pub const ATTESTATION_FILE: &str = "attestation.json";
/// Directory slot for a project under an identities root.
/// Slot thư mục của project dưới identities root.
pub fn attestation_dir(identities_dir: &Path, project_root: &Path) -> PathBuf {
    let canonical = std::fs::canonicalize(project_root)
        .unwrap_or_else(|_| project_root.to_path_buf())
        .to_string_lossy()
        .to_string();
    identities_dir.join(blake3_hex(canonical.as_bytes()))
}

/// Read `mgc.toml` bytes without following symlinks — fail-closed on
/// unreadable input (the caller decides warn vs fail).
/// Đọc bytes `mgc.toml` mà không đi theo symlink; dữ liệu không đọc được sẽ bị chặn.
pub fn read_config_bytes(project_root: &Path) -> anyhow::Result<Vec<u8>> {
    let path = project_root.join("mgc.toml");
    let text = crate::project::read_regular_project_text(&path, "project config")?
        .ok_or_else(|| anyhow::anyhow!("project has no mgc.toml"))?;
    Ok(text.into_bytes())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
