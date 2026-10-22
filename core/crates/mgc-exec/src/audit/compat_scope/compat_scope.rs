use std::path::Path;

use serde::Serialize;

use crate::audit::{append_serialized, now_ts};

/// Record the user-selected compatibility process-tree scope before spawn.
pub fn append_compat_scope(
    path: &Path,
    manager: &str,
    root_cmd: &str,
    root_args: &[String],
    cwd: &Path,
    dry_run: bool,
) -> anyhow::Result<()> {
    #[derive(Serialize)]
    struct CompatScopeEntry<'a> {
        event: &'static str,
        manager: &'a str,
        root_cmd: &'a str,
        root_args: &'a [String],
        cwd: String,
        scope: &'static str,
        dry_run: bool,
        ts: u64,
    }

    append_serialized(
        path,
        &CompatScopeEntry {
            event: "compat_runner_scope",
            manager,
            root_cmd,
            root_args,
            cwd: cwd.display().to_string(),
            scope: "root_process_tree",
            dry_run,
            ts: now_ts(),
        },
    )
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
