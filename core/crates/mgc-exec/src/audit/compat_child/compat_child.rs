use std::path::Path;

use serde::Serialize;

use crate::audit::{append_serialized, now_ts};

/// Record a package-manager process observed inside the declared compatibility tree.
pub fn append_compat_child(
    path: &Path,
    manager: &str,
    pid: u32,
    image: &str,
    program: &str,
    args: &[String],
    cwd: &Path,
) -> anyhow::Result<()> {
    #[derive(Serialize)]
    struct CompatChildEntry<'a> {
        event: &'static str,
        manager: &'a str,
        pid: u32,
        image: &'a str,
        program: &'a str,
        args: &'a [String],
        cwd: String,
        scope: &'static str,
        ts: u64,
    }

    append_serialized(
        path,
        &CompatChildEntry {
            event: "compat_runner_child_observed",
            manager,
            pid,
            image,
            program,
            args,
            cwd: cwd.display().to_string(),
            scope: "root_process_tree",
            ts: now_ts(),
        },
    )
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
