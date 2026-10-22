//! Executes install for a non-monorepo set of detected targets.
//! Thực thi install cho tập target không phải monorepo.

use anyhow::Result;

use super::super::super::super::compat_install_target;
use super::super::options::WebInstallOptions;
use super::super::plan::WebInstallPlan;
use crate::commands::core::shared;
use crate::commands::core::shared::should_use_legacy_flat_layout;

pub(in crate::commands::core::web::implementation::package_lifecycle::install) async fn run(
    plan: &WebInstallPlan,
    options: &WebInstallOptions,
) -> Result<()> {
    for target in &plan.targets {
        if target.join("package.json").exists() {
            shared::install_with_adapter(
                plan.adapter.as_ref(),
                target,
                "mgc add",
                options.frozen || options.offline,
                mgc_types::adapter::InstallOptions {
                    allow_scripts: options.allow_scripts,
                    prefer_dedupe: plan.dedupe_enabled,
                    repair: options.repair,
                    legacy_flat: should_use_legacy_flat_layout("web"),
                    offline: options.offline,
                    ..Default::default()
                },
            )
            .await?;
        } else {
            compat_install_target(target, &plan.compat)?;
        }
    }
    Ok(())
}
