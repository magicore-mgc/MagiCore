//! Native registry package installation and execution shared by CLI flows.
//! Bộ máy MGC dùng chung cho dlx, create initializer và các luồng chạy package khác.

use anyhow::{Result, bail};
use std::path::{Path, PathBuf};

const MAX_REGISTRY_PACKAGE_REQUEST_BYTES: usize = 4096;
const MAX_NPM_PACKAGE_NAME_BYTES: usize = 214;
const DLX_WORK_DIRECTORY_COMPONENTS: &[&str] = &["cache", "web", "dlx"];
const CREATE_WORK_DIRECTORY_COMPONENTS: &[&str] = &["cache", "web", "create-initializers"];
const DLX_WORK_PREFIX: &str = "run-";
const CREATE_WORK_PREFIX: &str = "initializer-";
const PACKAGE_RUNNER_PROJECT_NAME: &str = "magicore-package-runner";
const PACKAGE_RUNNER_PROJECT_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PackageRunPurpose {
    Dlx,
    CreateInitializer,
}

impl PackageRunPurpose {
    fn label(self) -> &'static str {
        match self {
            Self::Dlx => "dlx",
            Self::CreateInitializer => "create",
        }
    }

    fn work_directory_components(self) -> &'static [&'static str] {
        match self {
            Self::Dlx => DLX_WORK_DIRECTORY_COMPONENTS,
            Self::CreateInitializer => CREATE_WORK_DIRECTORY_COMPONENTS,
        }
    }

    fn work_prefix(self) -> &'static str {
        match self {
            Self::Dlx => DLX_WORK_PREFIX,
            Self::CreateInitializer => CREATE_WORK_PREFIX,
        }
    }
}

/// Resolved package identity after MGC install and execution.
/// Danh tính version package đã được MGC cài và chạy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RegistryPackageReport {
    pub package: String,
    pub version: String,
}

/// Install a registry package through MGC and run its declared binary in a caller-owned directory.
/// Cài package bằng engine MGC rồi chạy bin đã khai báo trong thư mục do luồng gọi quản lý.
pub(crate) async fn run_registry_package_in(
    package: String,
    selected_bin: Option<String>,
    allow_scripts: bool,
    args: Vec<String>,
    execution_cwd: &Path,
    purpose: PackageRunPurpose,
) -> Result<RegistryPackageReport> {
    let (pkg_name, version_tag) = parse_request(&package)?;
    if selected_bin
        .as_deref()
        .is_some_and(|name| !valid_bin_name(name))
    {
        return Err(crate::error::registry_package_invalid_bin());
    }
    let cwd = std::env::current_dir()?;
    let execution_metadata = std::fs::symlink_metadata(execution_cwd)?;
    if execution_metadata.file_type().is_symlink() || !execution_metadata.is_dir() {
        bail!("package execution working directory must be a real directory");
    }
    let execution_cwd = execution_cwd.canonicalize()?;
    let project_config = registry_project_config(&cwd)?;
    let work_root = work_root_at(&mgc_platform::paths::mgc_data_dir(), purpose)?;
    let workspace = tempfile::Builder::new()
        .prefix(purpose.work_prefix())
        .tempdir_in(&work_root)?;
    let project_root = workspace.path();
    let canonical_project = project_root.canonicalize()?;
    if !canonical_project.starts_with(&work_root) {
        return Err(crate::error::registry_package_cache_path_unsafe(project_root));
    }

    let pkg_json = serde_json::json!({
        "name": PACKAGE_RUNNER_PROJECT_NAME, "version": PACKAGE_RUNNER_PROJECT_VERSION,
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

    mgc_ui::info(&format!(
        "{}: installing {}@{} through MagiCore",
        purpose.label(),
        pkg_name,
        version_tag
    ));
    if allow_scripts {
        mgc_ui::warning(&format!(
            "{} lifecycle scripts are enabled and run with the current user's permissions",
            purpose.label()
        ));
    }
    let mgc_bin = std::env::current_exe()?;
    let install_args = registry_install_args(allow_scripts);
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
        return Err(crate::error::registry_package_binary_outside_project());
    }
    let installed_version = requested_package_version(&package_root, pkg_name)?;
    let entries = requested_package_bins(&package_root, pkg_name)?;
    let bin_name = select_bin_name(&entries, pkg_name, selected_bin.as_deref())?;
    let declared_target = entries
        .iter()
        .find(|(name, _)| name == &bin_name)
        .ok_or_else(crate::error::registry_package_undeclared_bin)?
        .1
        .clone();
    let canonical_target = package_root.join(declared_target).canonicalize()?;
    if !canonical_target.starts_with(&package_root) || !canonical_target.is_file() {
        return Err(crate::error::registry_package_bin_target_mismatch());
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
        return Err(crate::error::registry_package_no_binary(&bin_name, pkg_name));
    }
    if !bin_path.canonicalize()?.starts_with(&canonical_project) {
        return Err(crate::error::registry_package_binary_outside_project());
    }
    #[cfg(unix)]
    if bin_path.canonicalize()? != canonical_target {
        return Err(crate::error::registry_package_bin_target_mismatch());
    }
    mgc_ui::info(&format!(
        "{}: running {} binary {} {}",
        purpose.label(),
        if purpose == PackageRunPurpose::CreateInitializer {
            "framework initializer"
        } else {
            "package"
        },
        bin_path.display(),
        args.join(" ")
    ));
    let host_path = std::env::var_os("PATH").unwrap_or_default();
    let node_script = is_node_script(&canonical_target);
    let mut node_launcher = None;
    let (program, execution_args) = if node_script {
        // Some npm bin targets are valid JavaScript entrypoints without the
        // executable bit (notably Nest CLI). Launch them through the host Node
        // runtime instead of chmod'ing registry content or failing on execve.
        // Một số bin npm là JavaScript hợp lệ nhưng không có bit executable;
        // chạy bằng Node của máy thay vì sửa quyền file registry hoặc execve lỗi.
        let node = resolve_host_node_binary(&host_path)?;
        let launcher = create_node_launcher(&execution_cwd, &canonical_target)?;
        let mut execution_args = Vec::with_capacity(args.len() + 1);
        execution_args.push(launcher.path().to_string_lossy().into_owned());
        execution_args.extend(args);
        node_launcher = Some(launcher);
        (node, execution_args)
    } else {
        (bin_path.clone(), args)
    };
    let mut paths = if node_script {
        std::env::split_paths(&host_path).collect::<Vec<_>>()
    } else {
        vec![]
    };
    paths.push(bin_dir);
    if !node_script {
        paths.extend(std::env::split_paths(&host_path));
    }
    let path = std::env::join_paths(paths)?;
    let opts = mgc_exec::prelude::ExecOptions {
        cwd: Some(execution_cwd.clone()),
        log_path: Some(execution_cwd.join(".magicore").join("exec.log")),
        clean_env: true,
        env: vec![("PATH".to_string(), path.to_string_lossy().into_owned())],
        execution_scope: Some(mgc_exec::allowlist::ExecutionScope::ScriptRunner),
        ..Default::default()
    };
    // Keep the isolated materialization alive until the entire runner finishes.
    // Giữ project cài package và Node launcher đến khi initializer kết thúc.
    mgc_exec::prelude::run_project_binary_inherited(&program, &execution_args, &opts)?;
    drop(node_launcher);
    Ok(RegistryPackageReport {
        package: pkg_name.to_string(),
        version: installed_version,
    })
}

fn is_node_script(path: &Path) -> bool {
    path.extension()
        .and_then(std::ffi::OsStr::to_str)
        .is_some_and(|extension| {
            extension.eq_ignore_ascii_case("js")
                || extension.eq_ignore_ascii_case("mjs")
                || extension.eq_ignore_ascii_case("cjs")
        })
}

fn resolve_host_node_binary(path: &std::ffi::OsStr) -> Result<PathBuf> {
    for directory in std::env::split_paths(path).filter(|directory| directory.is_absolute()) {
        #[cfg(windows)]
        let candidates = [directory.join("node.exe")];
        #[cfg(not(windows))]
        let candidates = [directory.join("node")];
        for candidate in candidates {
            let Ok(metadata) = std::fs::metadata(&candidate) else {
                continue;
            };
            if !metadata.is_file() {
                continue;
            }
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if metadata.permissions().mode() & 0o111 == 0 {
                    continue;
                }
            }
            return candidate.canonicalize().map_err(Into::into);
        }
    }
    anyhow::bail!("a host Node.js runtime is required to execute this JavaScript package binary")
}

fn create_node_launcher(
    execution_cwd: &Path,
    entrypoint: &Path,
) -> Result<tempfile::NamedTempFile> {
    let entrypoint = entrypoint.to_string_lossy();
    let entrypoint_json = serde_json::to_string(entrypoint.as_ref())?;
    let source = format!(
        "import {{ pathToFileURL }} from 'node:url';\nconst entry = {entrypoint_json};\nprocess.argv[1] = entry;\nawait import(pathToFileURL(entry).href);\n"
    );
    let mut launcher = tempfile::Builder::new()
        .prefix(".mgc-create-initializer-")
        .suffix(".mjs")
        .tempfile_in(execution_cwd)?;
    std::io::Write::write_all(&mut launcher, source.as_bytes())?;
    Ok(launcher)
}

pub(crate) fn registry_install_args(allow_scripts: bool) -> Vec<String> {
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
pub(crate) fn registry_project_config(cwd: &Path) -> Result<mgc_config::project::ProjectConfig> {
    let mut config =
        mgc_config::project::ProjectConfig::new(PACKAGE_RUNNER_PROJECT_NAME, "web");
    config.version = PACKAGE_RUNNER_PROJECT_VERSION.to_string();
    if let Some(caller) = mgc_config::project::ProjectConfig::load(cwd)
        .map_err(|_| crate::error::registry_package_invalid_caller_policy())?
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
pub(crate) fn requested_package_bins(root: &Path, package: &str) -> Result<Vec<(String, PathBuf)>> {
    let text = mgc_config::project::read_regular_project_text(
        &root.join("package.json"),
        "installed package manifest",
    )?
    .ok_or_else(crate::error::registry_package_invalid_bins)?;
    let value: serde_json::Value =
        serde_json::from_str(&text).map_err(|_| crate::error::registry_package_invalid_bins())?;
    if value.get("name").and_then(serde_json::Value::as_str) != Some(package) {
        return Err(crate::error::registry_package_invalid_bins());
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
                    .ok_or_else(crate::error::registry_package_invalid_bins)
            })
            .collect(),
        _ => Err(crate::error::registry_package_invalid_bins()),
    }
}

pub(crate) fn requested_package_version(root: &Path, package: &str) -> Result<String> {
    let text = mgc_config::project::read_regular_project_text(
        &root.join("package.json"),
        "installed package manifest",
    )?
    .ok_or_else(crate::error::registry_package_invalid_bins)?;
    let value: serde_json::Value =
        serde_json::from_str(&text).map_err(|_| crate::error::registry_package_invalid_bins())?;
    if value.get("name").and_then(serde_json::Value::as_str) != Some(package) {
        return Err(crate::error::registry_package_invalid_bins());
    }
    let version = value
        .get("version")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(crate::error::registry_package_invalid_bins)?;
    mgc_types::Version::parse(version).map_err(|_| crate::error::registry_package_invalid_bins())?;
    Ok(version.to_string())
}
// Choose a declared bin deterministically; ambiguous packages need explicit selection.
// Chọn bin tường minh hoặc duy nhất, không đoán giữa các target khác nhau.
pub(crate) fn valid_bin_name(name: &str) -> bool {
    !name.is_empty()
        && name != "."
        && name != ".."
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
}

pub(crate) fn select_bin_name(
    entries: &[(String, PathBuf)],
    package: &str,
    selected: Option<&str>,
) -> Result<String> {
    if entries.iter().any(|(name, _)| !valid_bin_name(name))
        || selected.is_some_and(|name| !valid_bin_name(name))
    {
        return Err(crate::error::registry_package_invalid_bin());
    }
    if let Some(name) = selected {
        return entries
            .iter()
            .find(|(entry, _)| entry == name)
            .map(|(name, _)| name.clone())
            .ok_or_else(crate::error::registry_package_undeclared_bin);
    }
    let basename = package.rsplit('/').next().unwrap_or(package);
    if entries.iter().any(|(name, _)| name == basename) {
        return Ok(basename.into());
    }
    let Some((first_name, first_target)) = entries.first() else {
        return Err(crate::error::registry_package_undeclared_bin());
    };
    if entries.iter().all(|(_, target)| target == first_target) {
        return Ok(entries
            .iter()
            .fold(first_name, |best, (name, _)| best.min(name))
            .clone());
    }
    Err(crate::error::registry_package_ambiguous_bin())
}

pub(crate) fn parse_request(request: &str) -> Result<(&str, &str)> {
    if request.is_empty() || request.len() > MAX_REGISTRY_PACKAGE_REQUEST_BYTES {
        return Err(crate::error::registry_package_invalid_request(
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
        return Err(crate::error::registry_package_invalid_request(
            "invalid registry package name",
        ));
    }
    if selector.trim().is_empty()
        || !selector.is_ascii()
        || selector.bytes().any(|byte| {
            byte.is_ascii_control() || matches!(byte, b'/' | b'\\' | b':' | b'%' | b'#' | b'?')
        })
    {
        return Err(crate::error::registry_package_invalid_request(
            "expected a registry version, range, or tag; paths and URLs are unsupported",
        ));
    }
    Ok((name, selector))
}

fn work_root_at(data_root: &Path, purpose: PackageRunPurpose) -> Result<PathBuf> {
    std::fs::create_dir_all(data_root)?;
    let canonical_data_root = data_root.canonicalize()?;
    let mut path = canonical_data_root.clone();
    for component in purpose.work_directory_components() {
        path.push(component);
        match std::fs::symlink_metadata(&path) {
            Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
                return Err(crate::error::registry_package_cache_path_unsafe(&path));
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
            return Err(crate::error::registry_package_cache_path_unsafe(&path));
        }
    }
    Ok(path)
}

#[cfg(test)]
#[path = "test/dlx.rs"]
mod tests;
