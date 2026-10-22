//! Deduplicated and redacted compat-subtree audit recording.
//! Ghi audit subtree compat có chống trùng và redaction.

use super::super::CompatAuditContext;
use super::error::CompatAuditError;
use crate::audit::append_compat_child;
use crate::sanitizer::redact_args;
use std::ffi::OsString;

/// Record a matching package-manager process once when it appears in the
/// monitored subtree. Raw arguments are redacted before leaving this module.
/// (Ghi process manager khớp một lần trong subtree; raw args được redact tại đây.)
pub(in crate::run::implementation) fn audit_compat_process(
    audit: Option<&CompatAuditContext<'_>>,
    pid: u32,
    image: &str,
    command: &[OsString],
) -> Result<(), CompatAuditError> {
    let Some(audit) = audit else {
        return Ok(());
    };
    let command_line = command
        .iter()
        .map(|arg| arg.to_string_lossy())
        .collect::<Vec<_>>()
        .join(" ");
    if super::super::super::process_policy::forbidden_process_match_argv(image, command, &[])
        .is_none_or(|(name, _, _, _)| name != audit.manager)
    {
        return Ok(());
    }
    let key = (pid, command_line);
    let mut seen = audit
        .seen_processes
        .lock()
        .map_err(|_| CompatAuditError::DedupStatePoisoned)?;
    if !seen.insert(key) {
        return Ok(());
    }
    drop(seen);
    let program = command
        .first()
        .map(|arg| arg.to_string_lossy().into_owned())
        .unwrap_or_default();
    let args = command
        .iter()
        .skip(1)
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    append_compat_child(
        audit.log_path,
        audit.manager,
        pid,
        image,
        &program,
        &redact_args(&args),
        audit.cwd,
    )
    .map_err(CompatAuditError::WriteFailed)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
