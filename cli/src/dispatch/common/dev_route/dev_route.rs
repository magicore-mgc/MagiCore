//! Dev/build/flash arms for the common dispatcher.
//! Nhánh dev/build/flash cho dispatcher chung.

use anyhow::Result;

use crate::commands;

pub(crate) async fn dev(
    core: Option<&str>,
    host: Option<String>,
    port: Option<u16>,
    clear: bool,
    compat_runtime: Option<String>,
    compat_runner: Option<String>,
) -> Result<()> {
    commands::dev::run(
        core,
        host,
        port,
        clear,
        compat_runtime.as_deref(),
        compat_runner.as_deref(),
    )
    .await
}

pub(crate) async fn build(
    core: Option<&str>,
    target: Option<String>,
    compat_runtime: Option<String>,
    compat_runner: Option<String>,
    engine: Option<String>,
    allow_output_warnings: bool,
) -> Result<()> {
    commands::build::run(
        core,
        target,
        compat_runtime.as_deref(),
        compat_runner.as_deref(),
        engine.as_deref(),
        allow_output_warnings,
    )
    .await
}

#[cfg(feature = "iot")]
pub(crate) async fn flash(board: Option<String>, skip_build: bool) -> Result<()> {
    commands::core::dev::iot::flash(board.as_deref(), skip_build).await
}

#[cfg(not(feature = "iot"))]
pub(crate) async fn flash(_board: Option<String>, _skip_build: bool) -> Result<()> {
    Err(crate::error::core_not_in_build("iot"))
}

pub(crate) async fn run(
    script: String,
    args: Vec<String>,
    core: Option<&str>,
    compat_runtime: Option<String>,
    compat_runner: Option<String>,
) -> Result<()> {
    commands::run::run(
        script,
        args,
        core,
        compat_runtime.as_deref(),
        compat_runner.as_deref(),
    )
    .await
}

pub(crate) async fn test(
    args: Vec<String>,
    core: Option<&str>,
    compat_runtime: Option<String>,
    compat_runner: Option<String>,
) -> Result<()> {
    commands::test::test(
        args,
        core,
        compat_runtime.as_deref(),
        compat_runner.as_deref(),
    )
    .await
}
