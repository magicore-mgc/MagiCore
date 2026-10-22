use std::path::Path;

use serde::Serialize;

use crate::audit::{append_serialized, now_ts};

/// Data required to record a native-mode child denied for dependency mutation.
pub(crate) struct NativeChildDenial<'a> {
    pub root_cmd: &'a str,
    pub root_args: &'a [String],
    pub cwd: &'a Path,
    pub child_pid: u32,
    pub child_name: &'a str,
    pub child_image: &'a str,
    pub matched_by: &'a str,
    pub matched_operation: &'a str,
}

pub(crate) fn append_native_child_denial(
    path: &Path,
    denial: NativeChildDenial<'_>,
) -> anyhow::Result<()> {
    #[derive(Serialize)]
    struct NativeChildDenialEntry<'a> {
        event: &'static str,
        policy: &'static str,
        root_cmd: &'a str,
        root_args: &'a [String],
        child_pid: u32,
        child_name: &'a str,
        child_image: &'a str,
        matched_by: &'a str,
        matched_operation: &'a str,
        cwd: String,
        scope: &'static str,
        ts: u64,
    }

    append_serialized(
        path,
        &NativeChildDenialEntry {
            event: "native_dependency_mutation_denied",
            policy: "native-deny",
            root_cmd: denial.root_cmd,
            root_args: denial.root_args,
            child_pid: denial.child_pid,
            child_name: denial.child_name,
            child_image: denial.child_image,
            matched_by: denial.matched_by,
            matched_operation: denial.matched_operation,
            cwd: denial.cwd.display().to_string(),
            scope: "root_process_tree",
            ts: now_ts(),
        },
    )
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
