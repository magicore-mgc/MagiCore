//! `mgc add` (auto-detect core) — programmatic entry shared by the MCP
//! `mgc_add` tool. Delegates to the detected core's OWN add lane (the same
//! function the CLI runs) so the C0 ownership firewall gates uniformly —
//! a direct adapter call here would spawn toolchains with no gate, no
//! compat opt-in, no audit.
//! (Lối add dùng chung cho MCP: gọi đúng lane của core để gate C0.)

use crate::context::ProjectContext;
use anyhow::Result;

use super::dispatch::dispatch_add;
use super::types::AddRequest;

/// mgc add — add a dependency to the project
#[allow(dead_code)]
// Mirrors the public CLI flag surface — giữ nguyên ánh xạ trực tiếp với các cờ CLI công khai.
#[allow(clippy::too_many_arguments)]
pub async fn run(
    package: String,
    version: Option<String>,
    dev: bool,
    exact: bool,
    optional: bool,
    peer: bool,
    no_save: bool,
    global: bool,
    core: Option<&str>,
) -> Result<()> {
    run_many(
        vec![package],
        version,
        dev,
        exact,
        optional,
        peer,
        no_save,
        global,
        core,
        None,
    )
    .await
}

// Mirrors the public CLI flag surface — giữ nguyên ánh xạ trực tiếp với các cờ CLI công khai.
#[allow(clippy::too_many_arguments)]
pub async fn run_many(
    packages: Vec<String>,
    version: Option<String>,
    dev: bool,
    exact: bool,
    optional: bool,
    peer: bool,
    no_save: bool,
    global: bool,
    core: Option<&str>,
    compat_runtime: Option<String>,
) -> Result<()> {
    if global {
        return Err(crate::error::global_package_install_unsupported());
    }
    // Fail closed outside a project AND outside the firewall: the detected
    // core selects its own lane (each gates before any spawn).
    // `compat_runtime` is None for MCP today (Native mode), so delegated
    // lanes fail closed naming the required flag.
    // (Chặn ngoài project và ngoài tường lửa: lane của core sẽ gate.)
    let ctx = ProjectContext::load_with_core(core)?;
    let request = AddRequest {
        packages,
        version,
        dev,
        exact,
        optional,
        peer,
        no_save,
        global,
        compat_runtime,
    };
    dispatch_add(request, ctx.config.ecosystem.as_str()).await
}
