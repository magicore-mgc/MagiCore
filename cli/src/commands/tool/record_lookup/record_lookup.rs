use super::super::record_validation::read_record_if_present;
use super::super::types::ToolRecord;
use anyhow::{Result, bail};
use std::fs;
use std::path::{Path, PathBuf};

pub(in crate::commands::tool) fn find_record_by_binary(
    tools_root: &Path,
    binary: &str,
) -> Result<Option<(PathBuf, ToolRecord)>> {
    let entries = match fs::read_dir(tools_root) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    let mut found = None;
    for entry in entries {
        let entry = entry?;
        let kind = entry.file_type()?;
        if kind.is_symlink() || !kind.is_dir() {
            bail!(
                "managed tools directory contains an unsafe entry '{}'; refusing to follow it",
                entry.path().display()
            );
        }
        let root = entry.path();
        let Some(record) = read_record_if_present(&root)? else {
            continue;
        };
        if record.binary == binary {
            if found.is_some() {
                bail!(
                    "multiple managed tools claim executable '{}'; repair the tool registry before continuing",
                    binary
                );
            }
            found = Some((root, record));
        }
    }
    Ok(found)
}
