// Optimizer env loader — load .mgc-optimizer/*.env files for runtime config
// Bộ tải env từ optimizer — đọc file .mgc-optimizer/*.env cho runtime config

use crate::commands::optimizer::runtime_detect::DetectedRuntime;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Load optimizer env files for detected runtime
/// Tải các file env từ optimizer cho runtime được phát hiện
///
/// SAFETY: Only loads env files matching the detected runtime to avoid stale config
/// AN TOÀN: Chỉ tải file env khớp với runtime phát hiện được để tránh config cũ
pub fn load_optimizer_env(
    project_root: &Path,
    runtime: &DetectedRuntime,
) -> Result<HashMap<String, String>> {
    let optimizer_dir = project_root.join(".mgc-optimizer");
    let mut env_vars = HashMap::new();

    match fs::symlink_metadata(&optimizer_dir) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            return Err(crate::error::optimizer_config_directory_invalid(
                &optimizer_dir,
            ));
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(env_vars),
        Err(error) => {
            return Err(crate::error::optimizer_config_read_failed(&optimizer_dir).context(error));
        }
    }

    // Map runtime to env file(s)
    // Ánh xạ runtime sang file env
    let env_files = get_env_files_for_runtime(runtime);

    for (filename, runtime_name) in env_files {
        let env_file = optimizer_dir.join(filename);
        match fs::symlink_metadata(&env_file) {
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => {
                return Err(crate::error::optimizer_config_read_failed(&env_file).context(error));
            }
        }

        // Parse based on file extension — phân tích theo phần mở rộng file.
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

/// Get env file names for a detected runtime
/// Lấy tên file env cho runtime đã phát hiện
fn get_env_files_for_runtime(runtime: &DetectedRuntime) -> Vec<(&'static str, &'static str)> {
    match runtime {
        // Web runtimes — runtime web
        DetectedRuntime::Bun => vec![("bun_env.env", "Bun")],
        DetectedRuntime::Deno => vec![("deno_env.env", "Deno")],
        DetectedRuntime::NodeJs { .. } => vec![("node_env.env", "Node.js")],

        // AI runtimes — runtime AI
        DetectedRuntime::PythonPyTorch => vec![
            ("pytorch_runtime.env", "PyTorch"),
            ("pytorch_docker.env", "PyTorch Docker"),
        ],
        DetectedRuntime::RustCandle => vec![
            ("candle_runtime.env", "Candle"),
            ("candle_cargo.toml", "Candle Cargo"),
        ],
        DetectedRuntime::GoTensorFlow => {
            vec![("go_ai_runtime.env", "Go AI"), ("go_build.env", "Go Build")]
        }

        // Lib runtimes — runtime thư viện
        DetectedRuntime::RustLib => vec![("rust_cargo_profile.toml", "Rust Lib")],
        DetectedRuntime::GoLib => vec![("go_lib_runtime.env", "Go Lib")],
        DetectedRuntime::PythonLib => vec![("python_lib_runtime.env", "Python Lib")],
        DetectedRuntime::TypeScriptLib => vec![("typescript_lib_env.env", "TypeScript Lib")],

        // App runtimes — runtime ứng dụng
        DetectedRuntime::Flutter => vec![("flutter_env.env", "Flutter")],
        DetectedRuntime::ReactNative => vec![("react_native_env.env", "React Native")],
        DetectedRuntime::RustNative => vec![("rust_cargo_profile.toml", "Rust Native")],

        // Fallback — dự phòng
        DetectedRuntime::Unknown => vec![],
    }
}

/// Parse .env file format (KEY=value lines)
/// Phân tích định dạng file .env (dòng KEY=value)
pub(crate) fn parse_env_file(path: &Path) -> Result<HashMap<String, String>> {
    let content = read_optimizer_text(path)?;

    let mut vars = HashMap::new();

    for (line_index, line) in content.lines().enumerate() {
        let line = line.trim();

        // Skip empty lines and comments
        // Bỏ qua dòng trống và comment
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse KEY=value format
        // Phân tích định dạng KEY=value
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_string();
            let value = value.trim().to_string();

            let value = strip_env_quotes(&value, line_index + 1)?;

            vars.insert(key, value);
        }
    }

    Ok(vars)
}

/// Strip balanced outer quotes and reject malformed quoting without echoing values.
/// Bỏ quote ngoài cân bằng; quote lỗi trả số dòng nhưng không in nội dung env.
fn strip_env_quotes(value: &str, line_number: usize) -> Result<String> {
    let bytes = value.as_bytes();
    let first = bytes.first().copied();
    let last = bytes.last().copied();

    match (first, last) {
        (Some(b'"'), Some(b'"')) if value.len() >= 2 => Ok(value[1..value.len() - 1].to_string()),
        (Some(b'\''), Some(b'\'')) if value.len() >= 2 => Ok(value[1..value.len() - 1].to_string()),
        (Some(b'"' | b'\''), _) => Err(crate::error::optimizer_env_unmatched_quote(line_number)),
        _ => Ok(value.to_string()),
    }
}

/// Parse Cargo profile TOML format to extract RUSTFLAGS
/// Phân tích định dạng Cargo profile TOML để trích xuất RUSTFLAGS
fn parse_toml_file(path: &Path) -> Result<HashMap<String, String>> {
    let content = read_optimizer_text(path)?;

    let toml: toml::Value = toml::from_str(&content).map_err(|error| {
        crate::error::optimizer_config_setting_invalid(path, "TOML").context(error)
    })?;

    let mut vars = HashMap::new();

    let Some(root) = toml.as_table() else {
        return Err(crate::error::optimizer_config_setting_invalid(path, "root"));
    };

    if root
        .keys()
        .any(|key| !matches!(key.as_str(), "build" | "profile" | "env"))
    {
        return Err(crate::error::optimizer_config_setting_invalid(
            path,
            "top-level key",
        ));
    }

    // Cargo exposes profile/build overrides through documented environment
    // variables; passing the TOML wholesale could activate unrelated settings.
    // Cargo nhận profile/build qua env đã công bố; không nạp nguyên TOML để
    // tránh vô tình kích hoạt cấu hình khác.
    if let Some(env_table) = root.get("env") {
        let Some(env_table) = env_table.as_table() else {
            return Err(crate::error::optimizer_config_setting_invalid(path, "env"));
        };
        for (key, value) in env_table {
            let Some(value) = value.as_str() else {
                return Err(crate::error::optimizer_config_setting_invalid(
                    path,
                    "env value",
                ));
            };
            if reserved_optimizer_env_key(key) {
                return Err(crate::error::optimizer_config_setting_invalid(
                    path,
                    "reserved env key",
                ));
            }
            vars.insert(key.clone(), value.to_string());
        }
    }

    if let Some(build) = root.get("build") {
        let Some(build) = build.as_table() else {
            return Err(crate::error::optimizer_config_setting_invalid(
                path, "build",
            ));
        };
        if build
            .keys()
            .any(|key| !matches!(key.as_str(), "jobs" | "rustflags"))
        {
            return Err(crate::error::optimizer_config_setting_invalid(
                path,
                "build key",
            ));
        }
        if let Some(jobs) = build.get("jobs") {
            let Some(jobs) = jobs.as_integer().filter(|jobs| *jobs > 0) else {
                return Err(crate::error::optimizer_config_setting_invalid(
                    path,
                    "build.jobs",
                ));
            };
            vars.insert("CARGO_BUILD_JOBS".to_string(), jobs.to_string());
        }
        if let Some(rustflags) = build.get("rustflags") {
            let Some(rustflags) = rustflags.as_array() else {
                return Err(crate::error::optimizer_config_setting_invalid(
                    path,
                    "build.rustflags",
                ));
            };
            let rustflags = rustflags
                .iter()
                .map(|value| {
                    value.as_str().map(str::to_string).ok_or_else(|| {
                        crate::error::optimizer_config_setting_invalid(path, "build.rustflags item")
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            if !rustflags.is_empty() {
                vars.insert("RUSTFLAGS".to_string(), rustflags.join(" "));
            }
        }
    }

    if let Some(profiles) = root.get("profile") {
        let Some(profiles) = profiles.as_table() else {
            return Err(crate::error::optimizer_config_setting_invalid(
                path, "profile",
            ));
        };
        for (profile, settings) in profiles {
            if !matches!(profile.as_str(), "dev" | "release" | "test" | "bench") {
                return Err(crate::error::optimizer_config_setting_invalid(
                    path,
                    "profile name",
                ));
            }
            let Some(settings) = settings.as_table() else {
                return Err(crate::error::optimizer_config_setting_invalid(
                    path,
                    "profile table",
                ));
            };
            for (setting, value) in settings {
                let env_key = cargo_profile_env_key(profile, setting);
                let value = validate_profile_value(path, setting, value)?;
                vars.insert(env_key, value);
            }
        }
    }

    Ok(vars)
}

/// Read an optimizer file through the project no-follow reader.
/// Đọc file optimizer qua bộ đọc project có cấm theo symlink.
fn read_optimizer_text(path: &Path) -> Result<String> {
    match mgc_config::project::read_regular_project_text(path, "optimizer configuration") {
        Ok(Some(contents)) => Ok(contents),
        Ok(None) => Err(crate::error::optimizer_config_missing(path)),
        Err(error) => Err(crate::error::optimizer_config_read_failed(path).context(error)),
    }
}

/// Cargo-owned process and loader settings cannot be overridden from `[env]`.
/// Không cho `[env]` ghi đè biến điều khiển Cargo hoặc nạp mã động.
fn reserved_optimizer_env_key(key: &str) -> bool {
    let key = key.to_ascii_uppercase();
    matches!(
        key.as_str(),
        "PATH"
            | "HOME"
            | "USERPROFILE"
            | "CARGO_HOME"
            | "CARGO_TARGET_DIR"
            | "RUSTC"
            | "RUSTC_WRAPPER"
            | "RUSTC_WORKSPACE_WRAPPER"
            | "RUSTFLAGS"
            | "CARGO_ENCODED_RUSTFLAGS"
            | "NODE_OPTIONS"
            | "PYTHONPATH"
            | "PYTHONHOME"
            | "LD_PRELOAD"
            | "DYLD_INSERT_LIBRARIES"
    ) || key.starts_with("CARGO_BUILD_")
        || key.starts_with("CARGO_PROFILE_")
        || key.starts_with("LD_")
        || key.starts_with("DYLD_")
}

/// Convert a Cargo profile field into its supported environment key.
/// Chuyển trường Cargo profile thành tên biến môi trường được hỗ trợ.
fn cargo_profile_env_key(profile: &str, setting: &str) -> String {
    format!(
        "CARGO_PROFILE_{}_{}",
        profile.to_ascii_uppercase(),
        setting.replace('-', "_").to_ascii_uppercase()
    )
}

/// Validate the intentionally small profile surface before passing it to Cargo.
/// Kiểm tra whitelist profile nhỏ trước khi chuyển cấu hình cho Cargo.
fn validate_profile_value(path: &Path, setting: &str, value: &toml::Value) -> Result<String> {
    let valid = match setting {
        "opt-level" => {
            value
                .as_integer()
                .is_some_and(|level| (0..=3).contains(&level))
                || value
                    .as_str()
                    .is_some_and(|level| matches!(level, "s" | "z"))
        }
        "codegen-units" => value
            .as_integer()
            .is_some_and(|units| (1..=65_536).contains(&units)),
        "debug" => {
            value.as_bool().is_some()
                || value
                    .as_integer()
                    .is_some_and(|level| (0..=2).contains(&level))
                || value.as_str().is_some_and(|level| {
                    matches!(
                        level,
                        "none" | "limited" | "line-tables-only" | "line-directives-only" | "full"
                    )
                })
        }
        "lto" => {
            value.as_bool().is_some()
                || value
                    .as_str()
                    .is_some_and(|mode| matches!(mode, "thin" | "fat" | "off"))
        }
        "panic" => value
            .as_str()
            .is_some_and(|mode| matches!(mode, "abort" | "unwind")),
        _ => {
            return Err(crate::error::optimizer_config_setting_invalid(
                path,
                "profile key",
            ));
        }
    };
    if !valid {
        return Err(crate::error::optimizer_config_setting_invalid(
            path, setting,
        ));
    }
    Ok(value.to_string())
}

// Tests moved to test/env_loader.rs per RULE §5
// Tests đã chuyển sang test/env_loader.rs theo RULE §5
#[cfg(test)]
#[path = "test/env_loader.rs"]
mod tests;
