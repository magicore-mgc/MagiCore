use anyhow::Result;
use std::path::Path;

pub(in crate::commands::config) fn file_value(path: &Path, key: &str) -> Result<Option<String>> {
    let Some(content) = read_optional_config_text(path, ".npmrc")? else {
        return Ok(None);
    };
    Ok(content.lines().find_map(|line| {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            return None;
        }
        let (k, v) = line.split_once('=')?;
        (k.trim() == key).then(|| v.trim().to_string())
    }))
}

pub(in crate::commands::config) fn read_optional_config_text(
    path: &Path,
    display_name: &str,
) -> Result<Option<String>> {
    match std::fs::symlink_metadata(path) {
        Ok(_) => mgc_adapter_base::project_file::read_regular_text(path, display_name)
            .map(Some)
            .map_err(anyhow::Error::from),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error.into()),
    }
}

pub(in crate::commands::config) fn read_lines(path: &Path) -> Result<Vec<String>> {
    let Some(content) = read_optional_config_text(path, ".npmrc")? else {
        return Ok(Vec::new());
    };
    Ok(content.lines().map(|l| l.trim_end().to_string()).collect())
}
