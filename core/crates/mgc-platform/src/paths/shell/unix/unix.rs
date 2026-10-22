//! Select the Unix startup profile and coordinate its safe update.
//! Chọn profile khởi động Unix rồi điều phối cập nhật an toàn.

use std::path::Path;

use super::ensure_directory_chain::ensure_directory_chain;
use super::ensure_profile_entry::ensure_profile_entry;

/// Select and update the correct Unix shell startup file.
/// Chọn và cập nhật startup file phù hợp với shell Unix.
pub(in crate::paths::shell) fn ensure_bin_on_path(
    home: &Path,
    shell: &str,
    bin: &Path,
) -> anyhow::Result<()> {
    let shell_name = Path::new(shell)
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    let (profile, fish) = match shell_name {
        "fish" => {
            let directory = home.join(".config/fish/conf.d");
            ensure_directory_chain(home, &directory)?;
            (directory.join("magicore.fish"), true)
        }
        "zsh" => (home.join(".zprofile"), false),
        "bash" => {
            let login = home.join(".bash_profile");
            let profile = if std::fs::symlink_metadata(&login).is_ok() {
                login
            } else {
                home.join(".profile")
            };
            (profile, false)
        }
        _ => (home.join(".profile"), false),
    };
    ensure_profile_entry(&profile, bin, fish)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
