use anyhow::Result;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub(in crate::commands::tool) fn read_json_object(
    path: &Path,
    label: &str,
) -> Result<serde_json::Map<String, serde_json::Value>> {
    let content = mgc_config::project::read_regular_project_text(path, label)?
        .ok_or_else(|| anyhow::anyhow!("{label} '{}' is missing", path.display()))?;
    let value: serde_json::Value = serde_json::from_str(&content)?;
    value
        .as_object()
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("{label} '{}' must contain a JSON object", path.display()))
}
pub(in crate::commands::tool) fn write_new_private_file(
    path: &Path,
    contents: &[u8],
) -> Result<()> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600).custom_flags(libc::O_NOFOLLOW);
    }
    let mut file = options.open(path)?;
    file.write_all(contents)?;
    file.sync_all()?;
    Ok(())
}
