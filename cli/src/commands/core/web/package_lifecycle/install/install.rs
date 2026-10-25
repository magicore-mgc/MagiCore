//! Orders the Web install stages without owning their implementation details.
//! Sắp thứ tự các bước install Web, không chứa chi tiết của từng owner.

use anyhow::Result;

use super::super::super::{WebProjectMode, detect_project_mode};
use super::backend_install::install_if_applicable;
use super::monorepo_install;
use super::options::WebInstallOptions;
use super::plan;
use super::preflight;
use super::single_target_install;

#[allow(clippy::too_many_arguments)]
pub async fn install(
    packages: Vec<String>,
    frozen: bool,
    ignore_scripts: bool,
    allow_scripts: bool,
    prefer_dedupe: bool,
    repair: bool,
    offline: bool,
    compat_runtime: Option<String>,
) -> Result<()> {
    let incremental = packages.is_empty();
    let options = WebInstallOptions::new(
        packages,
        frozen,
        ignore_scripts,
        allow_scripts,
        prefer_dedupe,
        repair,
        offline,
        incremental,
        compat_runtime,
    );
    let root = preflight::prepare(&options)?;
    if install_if_applicable(&root, &options).await? {
        return Ok(());
    }
    let plan = plan::prepare(&root, &options).await?;
    if matches!(detect_project_mode(&plan.root)?, WebProjectMode::Monorepo) {
        monorepo_install::run(&plan, &options).await?;
    } else {
        single_target_install::run(&plan, &options).await?;
    }
    // Supply-chain evidence is part of a successful install, not an extra
    // command the user has to remember.
    // Bằng chứng chuỗi cung ứng là một phần của install thành công, không phải
    // lệnh phụ user phải tự nhớ.
    crate::commands::install::write_install_sbom(&plan.root).await?;
    Ok(())
}
