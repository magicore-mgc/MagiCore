//! `mgc install` clo — tách từ core/clo.rs (Phase 7 v5).

use anyhow::Result;

use super::super::super::dev::clo as clo_tools;
use super::super::super::shared;

pub async fn install(
    packages: Vec<String>,
    dry_run: bool,
    frozen: bool,
    compat_runtime: Option<String>,
) -> Result<()> {
    // All cloud dependency lifecycle operations pass the same owner gate;
    // only CDK/Pulumi with package.json use the embedded MGC JS engine.
    let root = shared::core_project_root("clo")?;
    // One mutation at a time per project.
    // Mỗi lúc một mutation cho mỗi project.
    let _install_guard =
        crate::commands::core::shared::project_install_guard::acquire(&root, "clo")?;
    let cloud_kind = clo_tools::cloud_type(&root)?;
    crate::commands::dep_gate::gate_cloud_project(
        &root,
        &cloud_kind,
        crate::commands::dep_gate::DepOp::Install,
        compat_runtime.as_deref(),
    )?;
    let adapter = crate::commands::core::cloud::adapter(&root)?;
    if dry_run {
        // PAR-08: a dry run must report what would actually happen. Reporting
        // success before resolving hid the most common failure — a package name
        // the registry cannot resolve — so the request is validated here and
        // nothing is written.
        // (PAR-08: dry-run phải báo đúng việc sẽ xảy ra. Báo thành công trước
        // khi resolve che mất lỗi phổ biến nhất — tên package registry không
        // resolve được — nên request được kiểm ở đây và không ghi gì.)
        mgc_ui::info(&format!("[dry-run] cloud project kind: {cloud_kind}"));
        if packages.is_empty() {
            mgc_ui::info("[dry-run] would re-resolve and install the current cloud dependency set");
            return Ok(());
        }
        for package in &packages {
            let name = mgc_types::PackageName::new(package).map_err(|error| {
                anyhow::anyhow!("[dry-run] '{package}' is not a valid package name: {error}")
            })?;
            let prepared = adapter
                .prepare_add(
                    &root,
                    &name,
                    None,
                    mgc_types::adapter::AddOptions::default(),
                )
                .await
                .map_err(|error| {
                    anyhow::anyhow!(
                        "[dry-run] '{package}' cannot be resolved, so install would fail: {error}"
                    )
                })?;
            mgc_ui::info(&format!(
                "[dry-run] would add {}@{}",
                name.as_str(),
                prepared.id.version()
            ));
        }
        mgc_ui::info(
            "[dry-run] would run MagiCore-managed dependency install; nothing was written",
        );
        return Ok(());
    }
    if !packages.is_empty() {
        shared::add(
            &*adapter, &root, packages, None, false, false, false, false, false, false, false,
        )
        .await?;
    }
    shared::install_with_adapter(
        &*adapter,
        &root,
        "mgc add",
        frozen,
        mgc_types::adapter::InstallOptions {
            legacy_flat: false,
            ..Default::default()
        },
    )
    .await
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
