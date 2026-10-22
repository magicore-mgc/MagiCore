//! User-facing `mgc dlx` invocation and package-runtime handoff.
//! Invocation CLI `mgc dlx` và chuyển tiếp sang package runtime.

use std::path::PathBuf;

use anyhow::Result;

use crate::commands::package_runtime::{PackageRunPurpose, run_registry_package_in};

use super::error::DlxError;

#[derive(Debug, PartialEq, Eq)]
struct DlxInvocation {
    package: String,
    selected_bin: Option<String>,
    allow_scripts: bool,
    args: Vec<String>,
    execution_cwd: PathBuf,
    purpose: PackageRunPurpose,
}

impl DlxInvocation {
    fn prepare(
        package: String,
        selected_bin: Option<String>,
        allow_scripts: bool,
        args: Vec<String>,
    ) -> std::result::Result<Self, DlxError> {
        let execution_cwd = std::env::current_dir().map_err(DlxError::CurrentDirectory)?;
        Ok(Self {
            package,
            selected_bin,
            allow_scripts,
            args,
            execution_cwd,
            purpose: PackageRunPurpose::Dlx,
        })
    }
}

pub async fn run(
    package: String,
    selected_bin: Option<String>,
    allow_scripts: bool,
    args: Vec<String>,
) -> Result<()> {
    let invocation = DlxInvocation::prepare(package, selected_bin, allow_scripts, args)?;
    run_registry_package_in(
        invocation.package,
        invocation.selected_bin,
        invocation.allow_scripts,
        invocation.args,
        &invocation.execution_cwd,
        invocation.purpose,
    )
    .await
    .map(|_| ())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
