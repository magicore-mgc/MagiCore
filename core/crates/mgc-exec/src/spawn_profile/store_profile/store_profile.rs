use std::path::Path;

use crate::spawn_profile::{SpawnProfileFile, profile_file_name};

/// Store a profile file (creates parent dirs).
/// Lưu file profile (tạo thư mục cha nếu thiếu).
pub fn store_profile(store_dir: &Path, file: &SpawnProfileFile) -> anyhow::Result<()> {
    std::fs::create_dir_all(store_dir)?;
    let path = store_dir.join(profile_file_name(
        &file.profile.framework,
        &file.profile.operation,
        &file.profile.os,
    ));
    let text = serde_json::to_string_pretty(file)?;
    std::fs::write(path, text)?;
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
