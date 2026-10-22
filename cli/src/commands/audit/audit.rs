//! Shared CLI audit dispatch and coordination.
//! Điều phối và dispatch audit CLI dùng chung.

use anyhow::Result;
use mgc_types::Ecosystem;
use mgc_types::adapter::AuditReport;

use super::{
    OutputFormat, StrictMode, ai, app, cicd, clo, enforce_audit_exit, game, hardware, iot, lib,
    print_audit_report, web,
};
use crate::context::ProjectContext;

/// Entry: detect core (via ProjectContext) → dispatch per-core → finisher.
/// Điểm vào: detect core qua ProjectContext → dispatch per-core → finisher.
pub async fn run(core: Option<&str>, fix: bool, format: Option<&str>) -> Result<()> {
    run_with_strictness(core, fix, format, StrictMode::from_env()).await
}

/// Run an audit with incomplete scanner coverage treated as a hard failure.
/// Chạy audit với coverage scanner chưa đầy đủ được xem là lỗi cứng.
pub(crate) async fn run_strict(core: Option<&str>, fix: bool, format: Option<&str>) -> Result<()> {
    run_with_strictness(core, fix, format, StrictMode::required()).await
}

async fn run_with_strictness(
    core: Option<&str>,
    fix: bool,
    format: Option<&str>,
    strict: StrictMode,
) -> Result<()> {
    let fmt = OutputFormat::from_flag(format)?;
    let ctx = ProjectContext::load_with_core(core)?;
    match ctx.adapter().name() {
        "web" => web::audit(ctx.adapter(), ctx.root(), fix, fmt, strict).await,
        "game" => game::audit(&ctx, fmt, strict).await,
        "ai" => ai::audit(&ctx, fmt, strict).await,
        // The cloud adapter's name() is "cloud"; the CLI core id is
        // "clo" — accept BOTH (the adapter name is the runtime truth,
        // the short id is the user-facing core key).
        // name() của adapter cloud là "cloud"; id core CLI là "clo" —
        // nhận cả hai (tên adapter là sự thật runtime, id ngắn là khóa
        // core phía người dùng).
        "clo" | "cloud" => clo::audit(&ctx, fmt, strict).await,
        "cicd" => cicd::audit(&ctx, fmt, strict).await,
        "iot" => iot::audit(&ctx, fmt, strict).await,
        "app" => app::audit(&ctx, fmt, strict).await,
        "lib" => lib::audit(&ctx, fmt, strict).await,
        "hardware" => hardware::audit(&ctx, fmt, strict).await,
        other => Err(crate::error::unknown_core(other)),
    }
}

/// Shared per-core body: run the adapter's real scanner, then enforce the
/// exit contract via the finisher. Per-core files call this so the CLI
/// layer holds zero business logic (Tech Lead §11).
/// Body chung từng core: chạy scanner thật của adapter rồi giữ exit
/// contract qua finisher — file per-core chỉ dispatch, không business logic.
pub(in crate::commands) async fn run_adapter_audit(ctx: &ProjectContext) -> Result<AuditReport> {
    ctx.adapter()
        .audit(ctx.root())
        .await
        .map_err(|e| anyhow::anyhow!("{} adapter audit failed: {e}", ctx.adapter().name()))
}

/// Finisher contract (shared by every core — no per-core copy-paste):
/// - Any confirmed finding → exit 1, even when coverage is Partial/Failed;
///   the report still carries UNVERIFIED so incomplete coverage is visible.
/// - Available + 0 findings → exit 0, "clean" printed.
/// - Other incomplete states without findings → UNVERIFIED: exit 0 locally,
///   exit 2 in strict mode.
/// - Machine formats (json/sarif/cyclonedx) print ONLY the payload —
///   the state travels INSIDE the payload (schema_version / UNVERIFIED
///   markers), never as extra terminal chatter.
///
/// Split into print + enforce so `--fix` flows can render a report,
/// mutate, re-audit, and THEN decide the exit (P0-1: enforcing inside
/// the print would make the fix path unreachable).
///
/// Hợp đồng finisher (chung mọi core — không copy-paste từng core):
/// tách in và quyết exit để flow --fix render → sửa → audit lại → exit.
pub(in crate::commands) async fn finish_and_print(
    core: &str,
    report: &AuditReport,
    strict: StrictMode,
    fmt: OutputFormat,
) -> Result<()> {
    print_audit_report(core, report, fmt)?;
    enforce_audit_exit(core, report, strict)
}

/// Legacy per-core entry for external callers that already hold an
/// Ecosystem (kept public for library consumers; the CLI itself and the
/// MCP server route through `run` directly).
/// Entry theo Ecosystem cho caller ngoài còn giữ Ecosystem (giữ public cho
/// library consumer; CLI và MCP gọi thẳng `run`).
#[allow(dead_code)]
pub async fn execute_audit(ecosystem: &Ecosystem) -> Result<()> {
    let core = match ecosystem {
        Ecosystem::Web => "web",
        Ecosystem::Game => "game",
        Ecosystem::Ai => "ai",
        Ecosystem::Cloud => "clo",
        Ecosystem::Cicd => "cicd",
        Ecosystem::Iot => "iot",
        Ecosystem::App => "app",
        Ecosystem::Lib => "lib",
        Ecosystem::Hardware => "hardware",
    };
    run(Some(core), false, None).await
}
