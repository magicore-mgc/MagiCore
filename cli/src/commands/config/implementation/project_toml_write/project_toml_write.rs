use super::super::atomic_write_config_file;
use crate::commands::config::implementation::display_config_value;
use crate::commands::config::implementation::find_mgc_toml;
use anyhow::Result;
use std::path::Path;

pub(in crate::commands::config) fn set_toml(key: &str, value: &str) -> Result<()> {
    let toml_path = find_mgc_toml()
        .ok_or_else(|| anyhow::anyhow!("mgc.toml not found — run `mgc init <core>` first"))?;
    set_toml_at(&toml_path, key, value)
}

pub(in crate::commands::config::implementation) fn set_toml_at(
    toml_path: &Path,
    key: &str,
    value: &str,
) -> Result<()> {
    let _project_lock = acquire_project_config_lock(toml_path)?;
    ensure_toml_key_mutable(key)?;
    ensure_regular_mgc_toml(toml_path)?;
    // Đọc raw TOML, ghi lại key theo dot-notation (vd: "ecosystem", "version", "mode")
    let content = mgc_adapter_base::project_file::read_regular_text(toml_path, "mgc.toml")?;
    let mut doc: toml_edit::DocumentMut = content.parse()?;
    // Hỗ trợ dot-notation: "game.engine" → doc["game"]["engine"]
    let parts: Vec<&str> = key.splitn(2, '.').collect();
    if parts.len() == 2 {
        let (table, field) = (parts[0], parts[1]);
        if doc.get(table).is_none() {
            doc[table] = toml_edit::table();
        }
        doc[table][field] = toml_edit::value(value);
    } else {
        doc[key] = toml_edit::value(value);
    }
    atomic_write_toml(toml_path, doc.to_string().as_bytes())?;
    let shown = display_config_value(key, value);
    mgc_ui::success(&format!("set {key} = {shown}  →  {}", toml_path.display()));
    Ok(())
}

pub(in crate::commands::config) fn delete_toml(key: &str) -> Result<()> {
    let toml_path = find_mgc_toml()
        .ok_or_else(|| anyhow::anyhow!("mgc.toml not found — run `mgc init <core>` first"))?;
    delete_toml_at(&toml_path, key)
}

pub(in crate::commands::config::implementation) fn delete_toml_at(
    toml_path: &Path,
    key: &str,
) -> Result<()> {
    let _project_lock = acquire_project_config_lock(toml_path)?;
    ensure_toml_key_mutable(key)?;
    ensure_regular_mgc_toml(toml_path)?;
    let content = mgc_adapter_base::project_file::read_regular_text(toml_path, "mgc.toml")?;
    let mut doc: toml_edit::DocumentMut = content.parse()?;
    let parts: Vec<&str> = key.splitn(2, '.').collect();
    if parts.len() == 2 {
        let (table, field) = (parts[0], parts[1]);
        // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
        if let Some(t) = doc.get_mut(table)
            && let Some(tbl) = t.as_table_like_mut()
        {
            tbl.remove(field);
        }
    } else {
        doc.remove(key);
    }
    atomic_write_toml(toml_path, doc.to_string().as_bytes())?;
    mgc_ui::success(&format!("unset {key}  →  {}", toml_path.display()));
    Ok(())
}

fn acquire_project_config_lock(
    toml_path: &Path,
) -> Result<mgc_lockfile::project_lock::ProjectWriteLock> {
    let project_root = toml_path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let lock = mgc_lockfile::project_lock::ProjectWriteLock::acquire(
        project_root,
        crate::commands::core::shared::writer_lock_timeout(project_root),
    )
    .map_err(|error| crate::error::config_project_lock_failed(project_root, &error))?;
    crate::commands::core::shared::ensure_no_pending_remove_journal(project_root, &lock)?;
    Ok(lock)
}

fn ensure_toml_key_mutable(key: &str) -> Result<()> {
    if key.split('.').next() == Some("ecosystem") {
        return Err(crate::error::config_core_identity_managed_by_signature());
    }
    Ok(())
}

fn ensure_regular_mgc_toml(path: &Path) -> Result<()> {
    let metadata = std::fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() {
        return Err(crate::error::config_project_file_symlink(path));
    }
    if !metadata.is_file() {
        anyhow::bail!("project config '{}' must be a regular file", path.display());
    }
    Ok(())
}

fn atomic_write_toml(path: &Path, bytes: &[u8]) -> Result<()> {
    ensure_regular_mgc_toml(path)?;
    atomic_write_config_file(path, bytes)
}
