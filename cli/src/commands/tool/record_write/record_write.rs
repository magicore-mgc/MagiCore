use super::super::atomic_file::atomic_replace_file;
use super::super::types::{PendingToolInstall, TOOL_MANIFEST, TOOL_PENDING, ToolRecord};
use anyhow::Result;
use std::path::Path;

pub(in crate::commands::tool) fn write_pending(
    root: &Path,
    pending: &PendingToolInstall,
) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(pending)?;
    atomic_replace_file(&root.join(TOOL_PENDING), &bytes)
}
pub(in crate::commands::tool) fn write_record(root: &Path, record: &ToolRecord) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(record)?;
    atomic_replace_file(&root.join(TOOL_MANIFEST), &bytes)
}
