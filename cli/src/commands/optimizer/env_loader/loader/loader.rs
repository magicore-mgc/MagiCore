//! Runtime-specific optimizer environment loading.
//! (Nạp biến môi trường optimizer theo runtime.)

use crate::commands::optimizer::env_loader::{
    cargo_toml::parse_toml_file, env_file::parse_env_file,
    project_files::validate_optimizer_directory, runtime_files::get_env_files_for_runtime,
};
use crate::commands::optimizer::runtime_detect::DetectedRuntime;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Load optimizer env files for the detected runtime.
/// Tải các file env optimizer phù hợp với runtime đã phát hiện.
///
/// SAFETY: Only matching runtime files are loaded, preventing stale settings.
/// AN TOÀN: Chỉ nạp file khớp runtime để tránh áp dụng cấu hình cũ.
pub fn load_optimizer_env(
    project_root: &Path,
    runtime: &DetectedRuntime,
) -> Result<HashMap<String, String>> {
    let optimizer_dir = project_root.join(".mgc-optimizer");
    let mut env_vars = HashMap::new();

    if !validate_optimizer_directory(&optimizer_dir)? {
        return Ok(env_vars);
    }

    // Map runtime to env file(s).
    // Ánh xạ runtime sang các file env tương ứng.
    let env_files = get_env_files_for_runtime(runtime);

    for (filename, runtime_name) in env_files {
        let env_file = optimizer_dir.join(filename);
        match fs::symlink_metadata(&env_file) {
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => {
                return Err(
                    crate::commands::optimizer::env_loader::error::config_read_failed(&env_file)
                        .context(error),
                );
            }
        }

        // Parse based on file extension.
        // Phân tích theo phần mở rộng của file.
        let vars = if filename.ends_with(".toml") {
            parse_toml_file(&env_file)
                .with_context(|| format!("Failed to parse {runtime_name} optimizer TOML"))?
        } else {
            parse_env_file(&env_file)
                .with_context(|| format!("Failed to parse {runtime_name} optimizer env file"))?
        };

        if !vars.is_empty() {
            mgc_ui::info(&format!(
                "Loaded {} optimizer config: {} variables",
                runtime_name,
                vars.len()
            ));
        }

        env_vars.extend(vars);
    }

    Ok(env_vars)
}
