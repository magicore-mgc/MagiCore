//! `mgc remove` (auto-detect core) — programmatic entry. Delegates to the
//! detected core's OWN remove lane (the same function the CLI runs) so the
//! C0 ownership firewall gates uniformly — a direct adapter call here
//! would spawn toolchains with no gate, no compat opt-in, no audit.
//! (Lối remove dùng chung: gọi đúng lane của core để gate C0.)

use crate::context::ProjectContext;
use anyhow::Result;

use super::dispatch::dispatch_remove;

/// mgc remove — remove a dependency from the project
#[allow(dead_code)]
pub async fn run(package: String, core: Option<&str>) -> Result<()> {
    run_many(vec![package], core, None).await
}

#[allow(dead_code)]
pub async fn run_many(
    packages: Vec<String>,
    core: Option<&str>,
    compat_runtime: Option<String>,
) -> Result<()> {
    // Fail closed outside a project AND outside the firewall: the detected
    // core selects its own lane (each gates before any spawn).
    // (Chặn ngoài project và ngoài tường lửa: lane của core sẽ gate.)
    let ctx = ProjectContext::load_with_core(core)?;
    dispatch_remove(packages, compat_runtime, ctx.config.ecosystem.as_str()).await
}
