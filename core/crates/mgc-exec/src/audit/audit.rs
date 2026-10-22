use std::path::Path;

use crate::audit::{AuditEntry, append_serialized};

/// Append one audit entry as a JSONL record.
pub fn append(path: &Path, entry: &AuditEntry) -> anyhow::Result<()> {
    append_serialized(path, entry)
}
