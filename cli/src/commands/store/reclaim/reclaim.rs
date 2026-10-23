//! Store reclamation: list unreferenced packages, then delete on explicit opt-in.
//! Thu hồi store: liệt kê package không còn tham chiếu, chỉ xoá khi opt-in rõ.

use anyhow::{Context, Result};
use std::path::Path;

use super::super::prune::prune_unreferenced;
use crate::commands::store::{ReclaimCandidate, ReclaimReport};

/// Reclaim is deliberately two-step: `--apply` builds the plan, `--yes`
/// executes it. Without both we only report, because deleting cache bytes is
/// not recoverable.
/// Reclaim cố ý hai bước: `--apply` lập kế hoạch, `--yes` mới xoá. Thiếu một
/// trong hai thì chỉ báo cáo, vì xoá cache không thể khôi phục.
pub(in crate::commands::store) fn run_reclaim(
    project_root: &Path,
    apply: bool,
    yes: bool,
    json: bool,
) -> Result<()> {
    let delete = apply && yes;
    let report = prune_unreferenced(project_root, !delete)?;
    let reclaim = ReclaimReport {
        candidates: report
            .unreferenced
            .iter()
            .map(|name| ReclaimCandidate {
                name: name.clone(),
                bytes: 0,
            })
            .collect(),
        total_bytes: report.removed_bytes,
        deleted: delete,
    };

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&reclaim).context("serialize reclaim report")?
        );
        return Ok(());
    }

    if reclaim.candidates.is_empty() {
        println!("No unreferenced packages in the store.");
        return Ok(());
    }

    println!(
        "{} package(s) no longer referenced by any project ({} bytes):",
        reclaim.candidates.len(),
        reclaim.total_bytes
    );
    for candidate in &reclaim.candidates {
        println!("  {} ({} bytes)", candidate.name, candidate.bytes);
    }
    if delete {
        println!("Deleted {} package(s).", report.removed.len());
    } else if apply {
        println!("Plan only: re-run with --apply --yes to delete.");
    } else {
        println!("Review only: re-run with --apply to plan deletion, then --apply --yes.");
    }
    println!(
        "Note: only byte-identical artifacts are reused from the store; a project that has not run install recently may not appear in the reference set."
    );
    Ok(())
}
