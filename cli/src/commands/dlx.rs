//! Native registry package execution through an isolated MGC-owned project.
//! Chạy package registry qua project tạm do MGC sở hữu, không tin thư mục cache tự khai.

use anyhow::Result;
use std::path::{Path, PathBuf};

const MAX_DLX_REQUEST_BYTES: usize = 4096;
const MAX_NPM_PACKAGE_NAME_BYTES: usize = 214;
const DLX_WORK_DIRECTORY_COMPONENTS: &[&str] = &["cache", "web", "dlx"];
const DLX_WORK_PREFIX: &str = "run-";
const DLX_PROJECT_NAME: &str = "magicore-dlx-env";
const DLX_PROJECT_VERSION: &str = "1.0.0";

/// Run a registry package after native resolution and verified materialization.
/// Artifact cache belongs to the common installer; an old executable directory
/// never grants permission to skip it.
/// Cài qua engine native mỗi lượt; tái dùng artifact đã kiểm trong store chung,
/// không dùng sự tồn tại của .bin làm chứng cứ nguồn gốc.
pub async fn run(
    package: String,
    selected_bin: Option<String>,
    allow_scripts: bool,
    args: Vec<String>,
) -> Result<()> {
    let (pkg_name, version_tag) = parse_request(&package)?;
    if selected_bin
        .as_deref()
        .is_some_and(|name| !valid_bin_name(name))
    {
        return Err(crate::error::dlx_invalid_bin());
    }
    let cwd = std::env::current_dir()?;
    let project_config = dlx_project_config(&cwd)?;
    let work_root = work_root_at(&mgc_platform::paths::mgc_data_dir())?;
    let workspace = tempfile::Builder::new()
        .prefix(DLX_WORK_PREFIX)
        .tempdir_in(&work_root)?;
    let project_root = workspace.path();
    let canonical_project = project_root.canonicalize()?;
    if !canonical_project.starts_with(&work_root) {
        return Err(crate::error::dlx_cache_path_unsafe(project_root));
    }

    let pkg_json = serde_json::json!({
        "name": DLX_PROJECT_NAME, "version": DLX_PROJECT_VERSION,
        "dependencies": {pkg_name: version_tag}
    });
    std::fs::write(
        project_root.join("package.json"),
        serde_json::to_string_pretty(&pkg_json)?,
    )?;
    std::fs::write(
        project_root.join("mgc.toml"),
        toml::to_string(&project_config)?,
    )?;
    // Validate the exact inherited snapshot before launching the installer.
    // Kiểm schema của chính snapshot đã ghi trước khi spawn installer.
    mgc_config::project::lock_schema_is_v4(project_root)?;
    // Preserve explicit project registry/auth configuration without logging it.
    // Giữ npmrc tường minh của caller trong project tạm; không in credential.
    if let Some(npmrc) =
        mgc_config::project::read_regular_project_text(&cwd.join(".npmrc"), ".npmrc")?
    {
        std::fs::write(project_root.join(".npmrc"), npmrc)?;
    }

    mgc_ui::info(&format!("dlx: installing {pkg_name}@{version_tag}"));
    if allow_scripts {
        mgc_ui::warning(
            "dlx lifecycle scripts are enabled and run with the current user's permissions",
        );
    }
    let mgc_bin = std::env::current_exe()?;
    let install_args = dlx_install_args(allow_scripts);
    let install_opts = mgc_exec::prelude::ExecOptions {
        cwd: Some(project_root.to_path_buf()),
        // The self-child needs the configured data/cache/registry environment.
        // Tiến trình MGC con phải giữ cấu hình data/cache/registry của caller.
        clean_env: false,
        execution_scope: Some(mgc_exec::allowlist::ExecutionScope::Install),
        ..Default::default()
    };
    mgc_exec::prelude::run_project_binary_inherited(&mgc_bin, &install_args, &install_opts)?;

    let bin_dir = project_root.join("node_modules").join(".bin");
    let package_root = project_root
        .join("node_modules")
        .join(pkg_name)
        .canonicalize()?;
    if !package_root.starts_with(&canonical_project) {
        return Err(crate::error::dlx_binary_outside_project());
    }
    let entries = requested_package_bins(&package_root, pkg_name)?;
    let bin_name = select_bin_name(&entries, pkg_name, selected_bin.as_deref())?;
    let declared_target = entries
        .iter()
        .find(|(name, _)| name == &bin_name)
        .ok_or_else(crate::error::dlx_undeclared_bin)?
        .1
        .clone();
    let canonical_target = package_root.join(declared_target).canonicalize()?;
    if !canonical_target.starts_with(&package_root) || !canonical_target.is_file() {
        return Err(crate::error::dlx_bin_target_mismatch());
    }
    let bin_path = bin_dir.join(&bin_name);
    #[cfg(windows)]
    let bin_path = {
        let windows_shim = bin_dir.join(format!("{bin_name}.cmd"));
        if windows_shim.is_file() {
            windows_shim
        } else {
            bin_path
        }
    };
    if !bin_path.is_file() {
        return Err(crate::error::dlx_no_binary(&bin_name, pkg_name));
    }
    if !bin_path.canonicalize()?.starts_with(&canonical_project) {
        return Err(crate::error::dlx_binary_outside_project());
    }
    #[cfg(unix)]
    if bin_path.canonicalize()? != canonical_target {
        return Err(crate::error::dlx_bin_target_mismatch());
    }
    mgc_ui::info(&format!("$ {} {}", bin_path.display(), args.join(" ")));
    let mut paths = vec![bin_dir];
    if let Some(path) = std::env::var_os("PATH") {
        paths.extend(std::env::split_paths(&path));
    }
    let path = std::env::join_paths(paths)?;
    let opts = mgc_exec::prelude::ExecOptions {
        cwd: Some(cwd.clone()),
        log_path: Some(cwd.join(".magicore").join("exec.log")),
        clean_env: true,
        env: vec![("PATH".to_string(), path.to_string_lossy().into_owned())],
        execution_scope: Some(mgc_exec::allowlist::ExecutionScope::ScriptRunner),
        ..Default::default()
    };
    // Keep the isolated materialization alive until the entire runner finishes.
    // Giữ project tạm đến khi runner kết thúc; drop sẽ dọn môi trường dlx.
    mgc_exec::prelude::run_project_binary_inherited(&bin_path, &args, &opts)?;
    Ok(())
}

pub(super) fn dlx_install_args(allow_scripts: bool) -> Vec<String> {
    let mut args = ["install", "--core", "web"]
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
    args.push(if allow_scripts {
        "--allow-scripts".to_string()
    } else {
        "--ignore-scripts".to_string()
    });
    args
}

// Preserve caller install policy without importing framework or compat execution settings.
// Giữ policy cài đặt của caller; project tạm vẫn thuộc web và không tự mở compat.
pub(super) fn dlx_project_config(cwd: &Path) -> Result<mgc_config::project::ProjectConfig> {
    let mut config = mgc_config::project::ProjectConfig::new(DLX_PROJECT_NAME, "web");
    config.version = DLX_PROJECT_VERSION.to_string();
    if let Some(caller) = mgc_config::project::ProjectConfig::load(cwd)
        .map_err(|_| crate::error::dlx_invalid_caller_policy())?
    {
        config.registries = caller.registries;
        config.security = caller.security;
        config.lock = caller.lock;
        config.trust = caller.trust;
    }
    Ok(config)
}

// Read only the installed requested package's executable declarations.
// Đọc bin của đúng package sau native install, không quét bin dependency.
pub(super) fn requested_package_bins(root: &Path, package: &str) -> Result<Vec<(String, PathBuf)>> {
    let text = mgc_config::project::read_regular_project_text(
        &root.join("package.json"),
        "installed package manifest",
    )?
    .ok_or_else(crate::error::dlx_invalid_package_bins)?;
    let value: serde_json::Value =
        serde_json::from_str(&text).map_err(|_| crate::error::dlx_invalid_package_bins())?;
    if value.get("name").and_then(serde_json::Value::as_str) != Some(package) {
        return Err(crate::error::dlx_invalid_package_bins());
    }
    match value.get("bin") {
        None | Some(serde_json::Value::Null) => Ok(Vec::new()),
        Some(serde_json::Value::String(target)) => Ok(vec![(
            package.rsplit('/').next().unwrap_or(package).to_string(),
            PathBuf::from(target),
        )]),
        Some(serde_json::Value::Object(entries)) => entries
            .iter()
            .map(|(name, target)| {
                target
                    .as_str()
                    .map(|target| (name.clone(), PathBuf::from(target)))
                    .ok_or_else(crate::error::dlx_invalid_package_bins)
            })
            .collect(),
        _ => Err(crate::error::dlx_invalid_package_bins()),
    }
}
// Choose a declared bin deterministically; ambiguous packages need explicit selection.
// Chọn bin tường minh hoặc duy nhất, không đoán giữa các target khác nhau.
pub(super) fn valid_bin_name(name: &str) -> bool {
    !name.is_empty()
        && name != "."
        && name != ".."
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
}

pub(super) fn select_bin_name(
    entries: &[(String, PathBuf)],
    package: &str,
    selected: Option<&str>,
) -> Result<String> {
    if entries.iter().any(|(name, _)| !valid_bin_name(name))
        || selected.is_some_and(|name| !valid_bin_name(name))
    {
        return Err(crate::error::dlx_invalid_bin());
    }
    if let Some(name) = selected {
        return entries
            .iter()
            .find(|(entry, _)| entry == name)
            .map(|(name, _)| name.clone())
            .ok_or_else(crate::error::dlx_undeclared_bin);
    }
    let basename = package.rsplit('/').next().unwrap_or(package);
    if entries.iter().any(|(name, _)| name == basename) {
        return Ok(basename.into());
    }
    let Some((first_name, first_target)) = entries.first() else {
        return Err(crate::error::dlx_undeclared_bin());
    };
    if entries.iter().all(|(_, target)| target == first_target) {
        return Ok(entries
            .iter()
            .fold(first_name, |best, (name, _)| best.min(name))
            .clone());
    }
    Err(crate::error::dlx_ambiguous_bin())
}

pub(super) fn parse_request(request: &str) -> Result<(&str, &str)> {
    if request.is_empty() || request.len() > MAX_DLX_REQUEST_BYTES {
        return Err(crate::error::dlx_invalid_request(
            "empty or oversized request",
        ));
    }
    let (name, selector) = match request.rfind('@') {
        Some(position) if position > 0 => (&request[..position], &request[position + 1..]),
        _ => (request, "latest"),
    };
    let valid_segment = |segment: &str| {
        !segment.is_empty()
            && !segment.starts_with(['.', '_'])
            && segment
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    };
    let valid_name = if let Some(scoped) = name.strip_prefix('@') {
        scoped
            .split_once('/')
            .is_some_and(|(scope, package)| valid_segment(scope) && valid_segment(package))
    } else {
        valid_segment(name)
    };
    if name.len() > MAX_NPM_PACKAGE_NAME_BYTES || !valid_name {
        return Err(crate::error::dlx_invalid_request(
            "invalid registry package name",
        ));
    }
    if selector.trim().is_empty()
        || !selector.is_ascii()
        || selector.bytes().any(|byte| {
            byte.is_ascii_control() || matches!(byte, b'/' | b'\\' | b':' | b'%' | b'#' | b'?')
        })
    {
        return Err(crate::error::dlx_invalid_request(
            "expected a registry version, range, or tag; paths and URLs are unsupported",
        ));
    }
    Ok((name, selector))
}

fn work_root_at(data_root: &Path) -> Result<PathBuf> {
    std::fs::create_dir_all(data_root)?;
    let canonical_data_root = data_root.canonicalize()?;
    let mut path = canonical_data_root.clone();
    for component in DLX_WORK_DIRECTORY_COMPONENTS {
        path.push(component);
        match std::fs::symlink_metadata(&path) {
            Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
                return Err(crate::error::dlx_cache_path_unsafe(&path));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                match std::fs::create_dir(&path) {
                    Ok(()) => {}
                    Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                    Err(error) => return Err(error.into()),
                }
            }
            Err(error) => return Err(error.into()),
        }
        let metadata = std::fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() || !metadata.is_dir() || path.canonicalize()? != path {
            return Err(crate::error::dlx_cache_path_unsafe(&path));
        }
    }
    Ok(path)
}

#[cfg(test)]
#[path = "test/dlx.rs"]
mod tests;
