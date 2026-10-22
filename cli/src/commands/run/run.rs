//! Run command entry point.
//! Điểm vào lệnh run.

use super::run_with_hooks;
use anyhow::Result;

/// mgc run <script> [args...] — MagiCore Native Task Runner.
/// Priority:
///   1. mgc.toml [scripts] section
///   2. package.json scripts (Web core only)
///
/// Native-engine ruling 2026-09-10: a script whose PROGRAM is a rival
/// JS runtime (bun/deno) or an external package manager (npm/npx/pnpm/
/// yarn/bunx) is REJECTED on the native path — no silent forwarding.
/// Historical `--compat-runtime bun|deno` flags are rejected; they never
/// delegate execution to a rival runtime.
/// Script có PROGRAM là runtime đối thủ (bun/deno) hoặc PM ngoài bị TỪ
/// CHỐI trên mọi đường — không chuyển tiếp âm thầm. Cờ tương thích cũ
/// không mở đường chạy runtime ngoài.
pub async fn run(
    script: String,
    args: Vec<String>,
    core: Option<&str>,
    compat_runtime: Option<&str>,
    compat_runner: Option<&str>,
) -> Result<()> {
    let compat = crate::commands::compat::CompatMode::from_flag(compat_runtime)?;
    // Delegated runner lane (Spec rev4 §7/§9): qualified only, warned loudly.
    // (Lane runner ủy quyền (Spec rev4 §7/§9): chỉ qualified, cảnh báo lớn.)
    let compat_runner = crate::commands::compat::parse_compat_runner(compat_runner)?;
    let ctx = crate::context::ProjectContext::load_with_core(core)?;
    let project_root = ctx.root();
    crate::commands::policy::refresh_url_manifest_if_configured(project_root).await;

    // Script source precedence (Spec rev4 §16.2 — decided): `mgc.toml`
    // wins over the ecosystem manifest; package.json is the fallback.
    // (Ưu tiên nguồn script (Spec rev4 §16.2 — đã chốt): `mgc.toml` thắng
    // manifest ecosystem; package.json là fallback.)
    run_with_hooks(
        project_root,
        &script,
        &args,
        &compat,
        compat_runner.as_deref(),
        "run",
    )
}
