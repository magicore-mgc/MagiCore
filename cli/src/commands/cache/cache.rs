//! Route the existing cache command operations.
//! Điều phối thao tác cache hiện có và giữ nguyên tham số CLI.

use super::{CacheAction, CacheTarget, cache_entries, clean, print_status, prune};
use anyhow::Result;

pub async fn run(
    action: String,
    target: String,
    yes: bool,
    dry_run: bool,
    core: Option<&str>,
) -> Result<()> {
    let action = CacheAction::parse(&action)?;
    let target = CacheTarget::parse(&target)?;
    let entries = cache_entries(target, core, action.includes_build_target(target))?;

    match action {
        CacheAction::Status => print_status(&entries),
        CacheAction::Clean => clean(&entries, yes),
        CacheAction::Prune => prune(&entries, yes, dry_run, core),
    }
}
