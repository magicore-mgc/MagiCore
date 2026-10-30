//! CLI entry point for `mgc optimizer`: resolve project context and run optimization.
//! Điểm vào CLI `mgc optimizer`: resolve context project rồi chạy tối ưu hóa.

use anyhow::Result;

/// CLI entry point for `mgc optimizer` command — điểm vào CLI cho lệnh `mgc optimizer`
/// Run optimizer on current project, auto-detect core from .mgc.core or use --core flag
pub async fn run(core: Option<&str>, force: bool) -> Result<()> {
    let ctx = crate::context::ProjectContext::load_with_core(core)?;
    let project_root = ctx.root();
    let ecosystem = ctx.adapter().ecosystem();
    // `Ecosystem::as_str()` IS the CLI core id (H-1): the hand-written match
    // here used to send `clo` projects in as `cloud`, which then needed an alias
    // in every consumer downstream.
    // (`Ecosystem::as_str()` chính là core id CLI; match viết tay trước đây đẩy
    // project `clo` vào dưới tên `cloud` và buộc downstream phải có alias.)
    let detected_core = ecosystem.as_str();

    mgc_ui::info(&format!(
        "Running MagiCore Optimizer for `{}` core in {}",
        detected_core,
        project_root.display()
    ));

    crate::commands::optimizer::optimize_project(project_root, detected_core, force)
}
