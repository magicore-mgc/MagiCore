//! Executes a verified package binary within the caller-owned project.
//! Chạy binary package đã xác minh trong project do caller sở hữu.

use anyhow::Result;
use std::ffi::{OsStr, OsString};
use std::path::Path;

use super::super::installed_binary::VerifiedBinary;
use super::super::work_root::PackageRunPurpose;

fn runner_path(bin_dir: &Path, host_path: &OsStr, node_script: bool) -> Result<OsString> {
    let mut paths = if node_script {
        std::env::split_paths(host_path).collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    paths.push(bin_dir.to_path_buf());
    if !node_script {
        paths.extend(std::env::split_paths(host_path));
    }
    Ok(std::env::join_paths(paths)?)
}

pub(in crate::commands::package_runtime) fn execute_verified_binary(
    installed: &VerifiedBinary,
    purpose: PackageRunPurpose,
    args: Vec<String>,
    execution_cwd: &Path,
) -> Result<()> {
    mgc_ui::info(&format!(
        "{}: running {} binary {} {}",
        purpose.label(),
        if purpose == PackageRunPurpose::CreateInitializer {
            "framework initializer"
        } else {
            "package"
        },
        installed.bin_path.display(),
        args.join(" ")
    ));
    let host_path = std::env::var_os("PATH").unwrap_or_default();
    let node_script = super::super::node_runtime::is_node_script(&installed.canonical_target);
    let mut node_launcher = None;
    let (program, execution_args) = if node_script {
        // Some npm bin targets are valid JavaScript entrypoints without the
        // executable bit (notably Nest CLI). Launch them through the host Node
        // runtime instead of chmod'ing registry content or failing on execve.
        // Một số bin npm là JavaScript hợp lệ nhưng không có bit executable;
        // chạy bằng Node của máy thay vì sửa quyền file registry hoặc execve lỗi.
        let node = super::super::node_runtime::resolve_host_node_binary(&host_path)?;
        let launcher = super::super::node_runtime::create_node_launcher(
            execution_cwd,
            &installed.canonical_target,
        )?;
        let mut execution_args = Vec::with_capacity(args.len() + 1);
        execution_args.push(launcher.path().to_string_lossy().into_owned());
        execution_args.extend(args);
        node_launcher = Some(launcher);
        (node, execution_args)
    } else {
        (installed.bin_path.clone(), args)
    };
    let path = runner_path(&installed.bin_dir, &host_path, node_script)?;
    let options = mgc_exec::prelude::ExecOptions {
        cwd: Some(execution_cwd.to_path_buf()),
        log_path: Some(execution_cwd.join(".magicore").join("exec.log")),
        clean_env: true,
        env: vec![("PATH".to_string(), path.to_string_lossy().into_owned())],
        execution_scope: Some(if purpose == PackageRunPurpose::CreateInitializer {
            mgc_exec::allowlist::ExecutionScope::FrameworkInitializer
        } else {
            mgc_exec::allowlist::ExecutionScope::ScriptRunner
        }),
        ..Default::default()
    };
    // Keep the isolated materialization alive until the entire runner finishes.
    // Giữ package cài cô lập đến khi runner kết thúc.
    mgc_exec::prelude::run_project_binary_inherited(&program, &execution_args, &options)?;
    drop(node_launcher);
    Ok(())
}

#[cfg(test)]
#[path = "test/execute_package.rs"]
mod tests;
