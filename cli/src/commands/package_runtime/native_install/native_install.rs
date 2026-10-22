//! Invokes the current MagiCore executable to install the requested registry package.
//! Gọi executable MagiCore hiện tại để cài package registry được yêu cầu.

use anyhow::Result;
use std::path::PathBuf;

use super::super::runner_workspace::RunnerWorkspace;
use super::super::work_root::PackageRunPurpose;

pub(super) struct NativeInstallInvocation {
    pub(super) binary: PathBuf,
    pub(super) args: Vec<String>,
    pub(super) options: mgc_exec::prelude::ExecOptions,
}

fn create_invocation(
    workspace: &RunnerWorkspace,
    allow_scripts: bool,
) -> Result<NativeInstallInvocation> {
    let binary = std::env::current_exe()?;
    let args = super::super::install_policy::registry_install_args(allow_scripts);
    let options = mgc_exec::prelude::ExecOptions {
        cwd: Some(workspace.project_root.clone()),
        // The self-child needs the configured data/cache/registry environment.
        // Tiến trình MGC con cần giữ cấu hình data/cache/registry của caller.
        clean_env: false,
        execution_scope: Some(mgc_exec::allowlist::ExecutionScope::Install),
        ..Default::default()
    };
    Ok(NativeInstallInvocation {
        binary,
        args,
        options,
    })
}

pub(in crate::commands::package_runtime) fn install_mgc_package(
    workspace: &RunnerWorkspace,
    package: &str,
    selector: &str,
    purpose: PackageRunPurpose,
    allow_scripts: bool,
) -> Result<()> {
    mgc_ui::info(&format!(
        "{}: installing {} {}@{} through MagiCore",
        purpose.label(),
        if purpose == PackageRunPurpose::CreateInitializer {
            "initializer"
        } else {
            "package"
        },
        package,
        selector
    ));
    if allow_scripts {
        mgc_ui::warning(&format!(
            "{} lifecycle scripts are enabled and run with the current user's permissions",
            purpose.label()
        ));
    }
    let invocation = create_invocation(workspace, allow_scripts)?;
    mgc_exec::prelude::run_project_binary_inherited(
        &invocation.binary,
        &invocation.args,
        &invocation.options,
    )?;
    Ok(())
}

#[cfg(test)]
#[path = "test/native_install.rs"]
mod tests;
