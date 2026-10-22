//! (Lệnh template: publish/fetch kernel-làm registry — Bun/pnpm create thành mgc-create-*, Q13)
//! publish: pack templates/{core}/{name} thành tarball mgc-create-<core>-<name>
//! fetch: tải tarball → ~/.mgc/templates/{core}/{name} (cache; resolve sẽ thấy Disk)

use crate::commands::template::{TemplateCmd, fetch, publish, publish_all};
use anyhow::Result;

/// Dispatch mgc template subcommands.
pub async fn run(cmd: TemplateCmd) -> Result<()> {
    match cmd {
        TemplateCmd::Fetch(args) => {
            fetch(args).await?;
        }
        TemplateCmd::Publish(args) => {
            publish(args).await?;
        }
        TemplateCmd::PublishAll => {
            publish_all().await?;
        }
    }
    Ok(())
}
