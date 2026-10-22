//! CLI entry point for `mgc optimizer`: resolve project context and run optimization.
//! Điểm vào CLI `mgc optimizer`: resolve context project rồi chạy tối ưu hóa.

use anyhow::Result;

/// CLI entry point for `mgc optimizer` command — điểm vào CLI cho lệnh `mgc optimizer`
/// Run optimizer on current project, auto-detect core from .mgc.core or use --core flag
pub async fn run(core: Option<&str>, force: bool) -> Result<()> {
    let ctx = crate::context::ProjectContext::load_with_core(core)?;
    let project_root = ctx.root();
    let ecosystem = ctx.adapter().ecosystem();
    let detected_core = match ecosystem {
        mgc_types::Ecosystem::Web => "web",
        mgc_types::Ecosystem::Game => "game",
        mgc_types::Ecosystem::Ai => "ai",
        mgc_types::Ecosystem::Cloud => "cloud",
        mgc_types::Ecosystem::Cicd => "cicd",
        mgc_types::Ecosystem::Iot => "iot",
        mgc_types::Ecosystem::App => "app",
        mgc_types::Ecosystem::Lib => "lib",
        mgc_types::Ecosystem::Hardware => "hardware",
    };

    mgc_ui::info(&format!(
        "Running MagiCore Optimizer for `{}` core in {}",
        detected_core,
        project_root.display()
    ));

    crate::commands::optimizer::optimize_project(project_root, detected_core, force)
}
