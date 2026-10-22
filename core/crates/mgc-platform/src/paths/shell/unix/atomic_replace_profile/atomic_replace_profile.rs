//! Persist a shell profile replacement through a synced temporary file.
//! Ghi profile qua file tạm đã sync rồi thay thế nguyên tử.

use std::path::Path;

#[cfg(not(windows))]
pub(in crate::paths::shell::unix) fn atomic_replace_profile(
    profile: &Path,
    content: &[u8],
    prior: Option<&std::fs::Metadata>,
) -> anyhow::Result<()> {
    use std::io::Write;

    let parent = profile
        .parent()
        .ok_or_else(|| anyhow::anyhow!("shell profile has no parent directory"))?;
    let temp = parent.join(format!(".magicore-path-{}.tmp", uuid_from_pid()));
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
        let mode = prior
            .map(|metadata| metadata.mode() & 0o777)
            .unwrap_or(0o600);
        options.mode(mode).custom_flags(libc::O_NOFOLLOW);
    }
    let result = (|| -> anyhow::Result<()> {
        let mut file = options.open(&temp)?;
        file.write_all(content)?;
        file.sync_all()?;
        replace_profile_file(&temp, profile)?;
        #[cfg(unix)]
        std::fs::File::open(parent)?.sync_all()?;
        Ok(())
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temp);
    }
    result
}

#[cfg(not(windows))]
fn replace_profile_file(temp: &Path, profile: &Path) -> anyhow::Result<()> {
    std::fs::rename(temp, profile)?;
    Ok(())
}

fn uuid_from_pid() -> String {
    format!(
        "{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    )
}
