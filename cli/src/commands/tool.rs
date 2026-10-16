//! Persistent, user-scoped CLI tools installed through MagiCore's native Web lane.
//! CLI bền vững theo user, cài qua Web adapter của MagiCore.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::commands::definitions::ToolCmd;
use crate::commands::package_runtime::{
    parse_request, registry_install_args, registry_project_config, requested_package_bins,
    select_bin_name, valid_bin_name,
};

const TOOL_MANIFEST: &str = ".mgc-tool.json";
const TOOL_PENDING: &str = ".mgc-tool-installing.json";
const TOOL_SCHEMA: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ToolRecord {
    schema: u32,
    source: String,
    package: String,
    selector: String,
    binary: String,
    target: String,
    installed_version: String,
    project_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PendingToolInstall {
    schema: u32,
    source: String,
    package: String,
    selector: String,
    project_name: String,
    record: Option<ToolRecord>,
}

pub fn run(command: ToolCmd) -> Result<()> {
    match command {
        ToolCmd::Install {
            package,
            bin,
            allow_scripts,
        } => install(&package, bin.as_deref(), allow_scripts),
        ToolCmd::Update {
            binary,
            version,
            allow_scripts,
        } => update(&binary, version.as_deref(), allow_scripts),
        ToolCmd::Remove { binary } => remove(&binary),
        ToolCmd::List => list(),
    }
}

fn install(request: &str, selected_bin: Option<&str>, allow_scripts: bool) -> Result<()> {
    require_web_lane()?;
    let Some(request) = request.strip_prefix("npm:") else {
        bail!(
            "global CLI tools require an explicit source, for example `npm:@openai/codex`; this release implements npm tools only"
        );
    };
    let (package, selector) = parse_request(request)?;
    if selected_bin.is_some_and(|binary| !valid_bin_name(binary)) {
        return Err(crate::error::registry_package_invalid_bin());
    }

    let (data_root, tools_root, bin_root) = managed_roots(true)?;
    let project_name = tool_project_name(package);
    let project_root = tools_root.join(&project_name);
    match fs::symlink_metadata(&project_root) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            bail!("managed tool path '{}' is unsafe", project_root.display())
        }
        Ok(_) => {
            if let Some(record) = read_record_if_present(&project_root)? {
                if record.package == package {
                    bail!(
                        "tool project already exists at '{}'; use `mgc tool update {}` or `mgc tool remove {}`",
                        project_root.display(),
                        record.binary,
                        record.binary
                    );
                }
                bail!("managed tool project identity does not match requested package");
            }
            let pending = read_pending_if_present(&project_root)?.ok_or_else(|| {
                anyhow::anyhow!(
                    "unrecognized or interrupted tool data at '{}'; inspect and remove it manually before retrying",
                    project_root.display()
                )
            })?;
            if pending.package != package {
                bail!("interrupted tool package does not match the requested package");
            }
            if let Some(record) = pending.record {
                let shim = shim_path(&bin_root, &record.binary);
                remove_owned_shim(&project_root, &record, &shim)?;
            }
            fs::remove_dir_all(&project_root)
                .context("remove interrupted managed tool installation before retrying")?;
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }
    fs::create_dir(&project_root)?;
    set_private_directory(&project_root)?;
    let mut pending = PendingToolInstall {
        schema: TOOL_SCHEMA,
        source: "npm".to_string(),
        package: package.to_string(),
        selector: selector.to_string(),
        project_name: project_name.clone(),
        record: None,
    };
    if let Err(error) = write_pending(&project_root, &pending) {
        let _ = fs::remove_dir_all(&project_root);
        return Err(error);
    }

    let result = install_into_project(
        &project_root,
        &data_root,
        package,
        selector,
        selected_bin,
        allow_scripts,
    );
    let record = match result {
        Ok(record) => record,
        Err(error) => {
            let _ = fs::remove_dir_all(&project_root);
            return Err(error);
        }
    };

    pending.record = Some(record.clone());
    if let Err(error) = write_pending(&project_root, &pending) {
        let shim = shim_path(&bin_root, &record.binary);
        let _ = remove_owned_shim(&project_root, &record, &shim);
        let _ = fs::remove_dir_all(&project_root);
        return Err(error);
    }

    let shim = shim_path(&bin_root, &record.binary);
    if let Err(error) = ensure_shim(&project_root, &record, &shim, false) {
        let _ = fs::remove_dir_all(&project_root);
        return Err(error);
    }
    if let Err(error) = write_record(&project_root, &record) {
        let _ = remove_owned_shim(&project_root, &record, &shim);
        let _ = fs::remove_dir_all(&project_root);
        return Err(error);
    }
    if let Err(error) = fs::remove_file(project_root.join(TOOL_PENDING)) {
        mgc_ui::warning(&format!(
            "tool is installed, but its recovery marker could not be removed: {error}"
        ));
    }
    if let Err(error) = mgc_platform::paths::ensure_global_bin_on_path() {
        let _ = remove_owned_shim(&project_root, &record, &shim);
        let _ = fs::remove_dir_all(&project_root);
        return Err(error).context(
            "tool installed, but user PATH registration failed; installation was rolled back",
        );
    }

    mgc_ui::success(&format!(
        "Installed {}@{} as '{}'. Open a new terminal to use it.",
        record.package, record.installed_version, record.binary
    ));
    Ok(())
}

fn install_into_project(
    project_root: &Path,
    data_root: &Path,
    package: &str,
    selector: &str,
    selected_bin: Option<&str>,
    allow_scripts: bool,
) -> Result<ToolRecord> {
    let cwd = std::env::current_dir()?;
    let caller_root =
        mgc_config::project::ProjectConfig::find_project_root(&cwd).unwrap_or_else(|| cwd.clone());
    let mut config = registry_project_config(&caller_root)?;
    config.name = tool_project_name(package);
    config.version = "1.0.0".to_string();
    for registry in &mut config.registries {
        registry.token = None;
        registry.username = None;
        registry.password = None;
    }
    config.save(project_root)?;

    let package_json = serde_json::json!({
        "name": config.name,
        "version": "1.0.0",
        "dependencies": { package: selector }
    });
    write_new_private_file(
        &project_root.join("package.json"),
        serde_json::to_vec_pretty(&package_json)?.as_slice(),
    )?;
    if allow_scripts {
        mgc_ui::warning(
            "tool lifecycle scripts are enabled and run with the current user's permissions",
        );
    }
    install_with_project_credentials(project_root, &caller_root, allow_scripts)?;

    let canonical_root = project_root.canonicalize()?;
    if !canonical_root.starts_with(data_root.join("tools").canonicalize()?) {
        bail!("managed tool project escaped the configured data directory");
    }
    let package_root = project_root
        .join("node_modules")
        .join(package)
        .canonicalize()?;
    if !package_root.starts_with(&canonical_root) {
        bail!("installed tool package escaped its managed project");
    }
    let entries = requested_package_bins(&package_root, package)?;
    let binary = select_bin_name(&entries, package, selected_bin)?;
    let target_relative = entries
        .iter()
        .find(|(name, _)| name == &binary)
        .map(|(_, target)| target.clone())
        .ok_or_else(crate::error::registry_package_undeclared_bin)?;
    let target = normalize_tool_target(&target_relative)?;
    let canonical_target = package_root.join(&target).canonicalize()?;
    if !canonical_target.starts_with(&package_root) || !canonical_target.is_file() {
        bail!("tool executable target is outside its installed package");
    }
    let installed_manifest = read_json_object(
        &package_root.join("package.json"),
        "installed package manifest",
    )?;
    let installed_version = installed_manifest
        .get("version")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| anyhow::anyhow!("installed tool package has no version"))?
        .to_string();
    Ok(ToolRecord {
        schema: TOOL_SCHEMA,
        source: "npm".to_string(),
        package: package.to_string(),
        selector: selector.to_string(),
        binary,
        target,
        installed_version,
        project_name: tool_project_name(package),
    })
}

fn update(binary: &str, selector_override: Option<&str>, allow_scripts: bool) -> Result<()> {
    require_web_lane()?;
    if !valid_bin_name(binary) {
        return Err(crate::error::registry_package_invalid_bin());
    }
    let (_data_root, tools_root, bin_root) = managed_roots(false)?;
    let (project_root, mut record) = find_record_by_binary(&tools_root, binary)?
        .ok_or_else(|| anyhow::anyhow!("managed tool '{}' is not installed", binary))?;
    let selector = selector_override.unwrap_or("latest");
    let request = format!("{}@{}", record.package, selector);
    let (parsed_package, parsed_selector) = parse_request(&request)?;
    if parsed_package != record.package {
        bail!("tool update package identity changed unexpectedly");
    }
    let old_record = record.clone();
    record.selector = parsed_selector.to_string();
    let package_json_path = project_root.join("package.json");
    let original_package_json = mgc_config::project::read_regular_project_text(
        &package_json_path,
        "managed tool package manifest",
    )?
    .ok_or_else(|| anyhow::anyhow!("managed tool package manifest is missing"))?;
    let mut package_json: serde_json::Value = serde_json::from_str(&original_package_json)?;
    let dependencies = package_json
        .get_mut("dependencies")
        .and_then(serde_json::Value::as_object_mut)
        .ok_or_else(|| anyhow::anyhow!("managed tool dependency manifest is malformed"))?;
    if !dependencies.contains_key(&record.package) {
        bail!("managed tool package manifest no longer owns the recorded package");
    }
    dependencies.insert(
        record.package.clone(),
        serde_json::Value::String(record.selector.clone()),
    );
    atomic_replace_file(
        &package_json_path,
        &serde_json::to_vec_pretty(&package_json)?,
    )?;

    let shim = shim_path(&bin_root, binary);
    let update_result = (|| -> Result<()> {
        let cwd = std::env::current_dir()?;
        let caller_root =
            mgc_config::project::ProjectConfig::find_project_root(&cwd).unwrap_or(cwd);
        install_with_project_credentials(&project_root, &caller_root, allow_scripts)?;
        let package_root = project_root
            .join("node_modules")
            .join(&record.package)
            .canonicalize()?;
        let project_root_canonical = project_root.canonicalize()?;
        if !package_root.starts_with(&project_root_canonical) {
            bail!("updated tool package escaped its managed project");
        }
        let entries = requested_package_bins(&package_root, &record.package)?;
        let target = entries
            .iter()
            .find(|(name, _)| name == binary)
            .map(|(_, target)| target.clone())
            .ok_or_else(crate::error::registry_package_undeclared_bin)?;
        let target = normalize_tool_target(&target)?;
        let canonical_target = package_root.join(&target).canonicalize()?;
        if !canonical_target.starts_with(&package_root) || !canonical_target.is_file() {
            bail!("updated tool executable target is outside its installed package");
        }
        let installed_manifest = read_json_object(
            &package_root.join("package.json"),
            "installed package manifest",
        )?;
        record.target = target;
        record.installed_version = installed_manifest
            .get("version")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("updated tool package has no version"))?
            .to_string();
        ensure_shim(&project_root, &record, &shim, true)?;
        write_record(&project_root, &record)?;
        Ok(())
    })();
    if let Err(error) = update_result {
        let restore = atomic_replace_file(&package_json_path, original_package_json.as_bytes());
        let rollback = if restore.is_ok() {
            let cwd = std::env::current_dir()?;
            let caller_root =
                mgc_config::project::ProjectConfig::find_project_root(&cwd).unwrap_or(cwd);
            install_with_project_credentials(&project_root, &caller_root, allow_scripts)
        } else {
            Ok(())
        };
        let restore_shim = ensure_shim(&project_root, &old_record, &shim, true);
        if let Err(restore_error) = restore {
            return Err(error).context(format!(
                "failed to restore tool manifest: {restore_error:#}"
            ));
        }
        if let Err(rollback_error) = rollback {
            return Err(error).context(format!(
                "tool update rollback also failed: {rollback_error:#}"
            ));
        }
        if let Err(shim_error) = restore_shim {
            return Err(error).context(format!("tool shim rollback also failed: {shim_error:#}"));
        }
        return Err(error).context("tool update failed; the previous version was restored");
    }
    mgc_ui::success(&format!(
        "Updated {} to {} ({})",
        record.binary, record.installed_version, record.package
    ));
    Ok(())
}

fn remove(binary: &str) -> Result<()> {
    if !valid_bin_name(binary) {
        return Err(crate::error::registry_package_invalid_bin());
    }
    let (_data_root, tools_root, bin_root) = managed_roots(false)?;
    let Some((project_root, record)) = find_record_by_binary(&tools_root, binary)? else {
        bail!("managed tool '{}' is not installed", binary);
    };
    let shim = shim_path(&bin_root, binary);
    remove_owned_shim(&project_root, &record, &shim)?;
    fs::remove_dir_all(&project_root)?;
    mgc_ui::success(&format!("Removed {} ({})", record.binary, record.package));
    Ok(())
}

fn list() -> Result<()> {
    let (_data_root, tools_root, _bin_root) = managed_roots(false)?;
    let entries = match fs::read_dir(&tools_root) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            println!("No managed CLI tools installed.");
            return Ok(());
        }
        Err(error) => return Err(error.into()),
    };
    let mut records = Vec::new();
    let mut pending_installs = Vec::new();
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
        match read_record_if_present(&root)? {
            Some(record) => records.push(record),
            None => match read_pending_if_present(&root)? {
                Some(pending) => pending_installs.push(pending),
                None => bail!(
                    "unrecognized managed tool data at '{}'; inspect it before removing it",
                    root.display()
                ),
            },
        }
    }
    records.sort_by(|left, right| left.binary.cmp(&right.binary));
    pending_installs.sort_by(|left, right| left.package.cmp(&right.package));
    if records.is_empty() && pending_installs.is_empty() {
        println!("No managed CLI tools installed.");
    } else {
        for record in records {
            println!(
                "{}\t{}:{}@{}",
                record.binary, record.source, record.package, record.installed_version
            );
        }
        for pending in pending_installs {
            println!(
                "INCOMPLETE\t{}:{}@{}\tretry `mgc tool install npm:{}@{}` to recover",
                pending.source,
                pending.package,
                pending.selector,
                pending.package,
                pending.selector
            );
        }
    }
    Ok(())
}

fn run_native_install(project_root: &Path, allow_scripts: bool) -> Result<()> {
    let args = registry_install_args(allow_scripts);
    let executable = std::env::current_exe()?;
    let options = mgc_exec::prelude::ExecOptions {
        cwd: Some(project_root.to_path_buf()),
        clean_env: false,
        execution_scope: Some(mgc_exec::allowlist::ExecutionScope::Install),
        ..Default::default()
    };
    mgc_exec::prelude::run_project_binary_inherited(&executable, &args, &options)?;
    Ok(())
}

fn require_web_lane() -> Result<()> {
    if crate::factory::available_cores()
        .iter()
        .all(|(core, _)| *core != "web")
    {
        return Err(crate::error::core_not_in_build("web"));
    }
    Ok(())
}

fn install_with_project_credentials(
    project_root: &Path,
    caller_root: &Path,
    allow_scripts: bool,
) -> Result<()> {
    let source = caller_root.join(".npmrc");
    let destination = project_root.join(".npmrc");
    let copied =
        if let Some(content) = mgc_config::project::read_regular_project_text(&source, ".npmrc")? {
            write_new_private_file(&destination, content.as_bytes())?;
            true
        } else {
            false
        };
    let install = run_native_install(project_root, allow_scripts);
    let cleanup = if copied {
        fs::remove_file(&destination).context("remove temporary tool registry credentials")
    } else {
        Ok(())
    };
    match (install, cleanup) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(error), Ok(())) => Err(error),
        (Ok(()), Err(error)) => Err(error),
        (Err(error), Err(cleanup_error)) => {
            Err(error).context(format!("credential cleanup also failed: {cleanup_error:#}"))
        }
    }
}

fn managed_roots(create: bool) -> Result<(PathBuf, PathBuf, PathBuf)> {
    let data = mgc_platform::paths::mgc_data_dir();
    if create {
        fs::create_dir_all(&data)?;
    }
    let data_root = match data.canonicalize() {
        Ok(path) => path,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && !create => {
            return Ok((data.clone(), data.join("tools"), data.join("bin")));
        }
        Err(error) => return Err(error.into()),
    };
    if !fs::symlink_metadata(&data_root)?.is_dir() {
        bail!("MGC_DATA_DIR '{}' is not a directory", data_root.display());
    }
    let tools = ensure_child_directory(&data_root, "tools", create)?;
    let bin = ensure_child_directory(&data_root, "bin", create)?;
    Ok((data_root, tools, bin))
}

fn ensure_child_directory(parent: &Path, name: &str, create: bool) -> Result<PathBuf> {
    let path = parent.join(name);
    match fs::symlink_metadata(&path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            bail!(
                "managed directory '{}' must be a real directory",
                path.display()
            )
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && create => {
            fs::create_dir(&path)?;
            set_private_directory(&path)?;
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(path),
        Err(error) => return Err(error.into()),
    }
    let canonical = path.canonicalize()?;
    if canonical != path {
        bail!(
            "managed directory '{}' resolves outside its expected path",
            path.display()
        );
    }
    Ok(path)
}

fn set_private_directory(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))?;
    }
    Ok(())
}

fn tool_project_name(package: &str) -> String {
    let hash = blake3::hash(package.as_bytes()).to_hex();
    format!("mgc-tool-{}", &hash[..24])
}

fn shim_path(bin_root: &Path, binary: &str) -> PathBuf {
    #[cfg(windows)]
    {
        bin_root.join(format!("{binary}.cmd"))
    }
    #[cfg(not(windows))]
    {
        bin_root.join(binary)
    }
}

fn ensure_shim(
    project_root: &Path,
    record: &ToolRecord,
    shim: &Path,
    replace_owned: bool,
) -> Result<()> {
    let bin_dir = project_root.join("node_modules/.bin");
    let executable = shimmed_binary_path(&bin_dir, &record.binary)?;
    let root = project_root.canonicalize()?;
    let local_target = executable.canonicalize()?;
    if !local_target.starts_with(&root) || !local_target.is_file() {
        bail!("tool shim target is outside the managed project");
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = fs::metadata(&local_target)?;
        if metadata.permissions().mode() & 0o111 == 0 {
            bail!(
                "package executable '{}' is not marked executable",
                local_target.display()
            );
        }
    }

    #[cfg(unix)]
    {
        match fs::symlink_metadata(shim) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                let existing = fs::read_link(shim)?;
                if existing == executable {
                    return Ok(());
                }
                if !replace_owned {
                    bail!(
                        "global executable '{}' already exists; refusing to overwrite it",
                        shim.display()
                    );
                }
                bail!(
                    "global executable '{}' is no longer owned by this tool; refusing to replace it",
                    shim.display()
                );
            }
            Ok(_) => bail!(
                "global executable '{}' already exists; refusing to overwrite it",
                shim.display()
            ),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        std::os::unix::fs::symlink(&executable, shim)?;
    }
    #[cfg(windows)]
    {
        // Use the package's declared target, not the generated node_modules/.bin
        // `.cmd` shim. Passing a `.cmd` file to `node` creates an invalid nested shim.
        // Dùng target khai báo của package, không dùng shim `.cmd` trong .bin;
        // truyền file `.cmd` cho `node` sẽ tạo shim lồng sai.
        let package_root = project_root.join("node_modules").join(&record.package);
        let package_target = package_root.join(&record.target).canonicalize()?;
        if !package_target.starts_with(package_root.canonicalize()?) || !package_target.is_file() {
            bail!("package executable target is outside the installed package");
        }
        let contents = windows_shim_contents(project_root, record)?;
        if shim.exists() && !replace_owned {
            bail!(
                "global executable '{}' already exists; refusing to overwrite it",
                shim.display()
            );
        }
        atomic_replace_file(shim, contents.as_bytes())?;
    }
    Ok(())
}

fn shimmed_binary_path(bin_dir: &Path, binary: &str) -> Result<PathBuf> {
    #[cfg(windows)]
    let path = bin_dir.join(format!("{binary}.cmd"));
    #[cfg(not(windows))]
    let path = bin_dir.join(binary);
    let metadata =
        fs::symlink_metadata(&path).context("package does not expose the selected executable")?;
    if !metadata.file_type().is_symlink() && !metadata.is_file() {
        bail!(
            "package executable shim '{}' is not a regular file or symlink",
            path.display()
        );
    }
    Ok(path)
}

fn remove_owned_shim(project_root: &Path, record: &ToolRecord, shim: &Path) -> Result<()> {
    match fs::symlink_metadata(shim) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.into()),
        Ok(_) => {}
    }
    #[cfg(unix)]
    {
        let expected =
            shimmed_binary_path(&project_root.join("node_modules/.bin"), &record.binary)?;
        let actual = fs::read_link(shim)?;
        if actual != expected {
            bail!(
                "global executable '{}' is not owned by this tool; refusing to remove it",
                shim.display()
            );
        }
        fs::remove_file(shim)?;
    }
    #[cfg(windows)]
    {
        let expected = windows_shim_contents(project_root, record)?;
        let actual = mgc_config::project::read_regular_project_text(shim, "managed tool shim")?
            .ok_or_else(|| anyhow::anyhow!("managed tool shim disappeared"))?;
        if actual != expected {
            bail!(
                "global executable '{}' is not owned by this tool; refusing to remove it",
                shim.display()
            );
        }
        fs::remove_file(shim)?;
    }
    Ok(())
}

#[cfg(windows)]
fn windows_shim_contents(project_root: &Path, record: &ToolRecord) -> Result<String> {
    let executable = project_root
        .join("node_modules")
        .join(&record.package)
        .join(&record.target)
        .canonicalize()?;
    render_windows_shim_contents(&executable)
}

#[cfg(any(windows, test))]
fn render_windows_shim_contents(executable: &Path) -> Result<String> {
    let path_text = executable.to_string_lossy();
    if path_text.contains('%') || path_text.chars().any(char::is_control) {
        bail!("Windows tool path contains characters unsafe for a command shim");
    }
    let command = if executable
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("exe"))
    {
        format!("\"{}\" %*", path_text)
    } else {
        format!("node \"{}\" %*", path_text)
    };
    Ok(format!("@echo off\r\n{command}\r\n"))
}

#[cfg(test)]
#[path = "test/tool.rs"]
mod tests;

fn find_record_by_binary(tools_root: &Path, binary: &str) -> Result<Option<(PathBuf, ToolRecord)>> {
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

fn read_record_if_present(root: &Path) -> Result<Option<ToolRecord>> {
    let path = root.join(TOOL_MANIFEST);
    let Some(content) =
        mgc_config::project::read_regular_project_text(&path, "managed tool record")?
    else {
        return Ok(None);
    };
    let record: ToolRecord = serde_json::from_str(&content)
        .with_context(|| format!("managed tool record '{}' is malformed", path.display()))?;
    validate_tool_record(root, &record)?;
    Ok(Some(record))
}

fn read_pending_if_present(root: &Path) -> Result<Option<PendingToolInstall>> {
    let path = root.join(TOOL_PENDING);
    let Some(content) =
        mgc_config::project::read_regular_project_text(&path, "managed tool recovery marker")?
    else {
        return Ok(None);
    };
    let pending: PendingToolInstall = serde_json::from_str(&content).with_context(|| {
        format!(
            "managed tool recovery marker '{}' is malformed",
            path.display()
        )
    })?;
    let request = format!("{}@{}", pending.package, pending.selector);
    let identity_valid = pending.schema == TOOL_SCHEMA
        && pending.source == "npm"
        && parse_request(&request).is_ok_and(|(package, _)| package == pending.package)
        && pending.project_name == tool_project_name(&pending.package)
        && root.file_name().and_then(|name| name.to_str()) == Some(&pending.project_name);
    if !identity_valid {
        bail!(
            "managed tool recovery marker '{}' has an unsupported schema or identity",
            path.display()
        );
    }
    if let Some(record) = &pending.record {
        validate_tool_record(root, record)?;
        if record.source != pending.source
            || record.package != pending.package
            || record.selector != pending.selector
            || record.project_name != pending.project_name
        {
            bail!(
                "managed tool recovery marker '{}' does not match its install record",
                path.display()
            );
        }
    }
    Ok(Some(pending))
}

fn validate_tool_record(root: &Path, record: &ToolRecord) -> Result<()> {
    let package_request = format!("{}@{}", record.package, record.selector);
    let package_valid =
        parse_request(&package_request).is_ok_and(|(package, _)| package == record.package);
    let target_valid = normalize_tool_target(Path::new(&record.target)).is_ok();
    if record.schema != TOOL_SCHEMA
        || record.source != "npm"
        || !valid_bin_name(&record.binary)
        || !package_valid
        || !target_valid
        || record.project_name != tool_project_name(&record.package)
        || root.file_name().and_then(|name| name.to_str()) != Some(&record.project_name)
    {
        bail!(
            "managed tool data at '{}' has an unsupported schema or identity",
            root.display()
        );
    }
    Ok(())
}

fn normalize_tool_target(target: &Path) -> Result<String> {
    // npm bin paths are portable manifest values; normalize both separator
    // styles before validating so records behave identically across OSes.
    // Đường dẫn bin của npm là dữ liệu manifest đa nền tảng; chuẩn hóa hai
    // kiểu dấu phân cách trước khi kiểm tra để record nhất quán mọi OS.
    let portable = target.to_string_lossy().replace('\\', "/");
    if portable.is_empty() || portable.starts_with('/') {
        return Err(crate::error::registry_package_invalid_bins());
    }
    let mut components = Vec::new();
    for component in portable.split('/') {
        match component {
            "" | "." => {}
            ".." => return Err(crate::error::registry_package_invalid_bins()),
            component if component.contains(':') || component.chars().any(char::is_control) => {
                return Err(crate::error::registry_package_invalid_bins());
            }
            component => components.push(component),
        }
    }
    if components.is_empty() {
        return Err(crate::error::registry_package_invalid_bins());
    }
    Ok(components.join("/"))
}

fn write_pending(root: &Path, pending: &PendingToolInstall) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(pending)?;
    atomic_replace_file(&root.join(TOOL_PENDING), &bytes)
}

fn write_record(root: &Path, record: &ToolRecord) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(record)?;
    atomic_replace_file(&root.join(TOOL_MANIFEST), &bytes)
}

fn read_json_object(
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

fn write_new_private_file(path: &Path, contents: &[u8]) -> Result<()> {
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

fn atomic_replace_file(path: &Path, contents: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("managed file '{}' has no parent", path.display()))?;
    let temp = parent.join(format!(
        ".mgc-tool-{}-{}.tmp",
        std::process::id(),
        uuid_suffix()
    ));
    write_new_private_file(&temp, contents)?;
    #[cfg(windows)]
    if path.exists() {
        let backup = path.with_extension(format!("mgc-old-{}", uuid_suffix()));
        fs::rename(path, &backup)?;
        if let Err(error) = fs::rename(&temp, path) {
            let _ = fs::rename(&backup, path);
            let _ = fs::remove_file(&temp);
            return Err(error.into());
        }
        fs::remove_file(backup)?;
        return Ok(());
    }
    fs::rename(&temp, path)?;
    #[cfg(unix)]
    fs::File::open(parent)?.sync_all()?;
    Ok(())
}

fn uuid_suffix() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}
