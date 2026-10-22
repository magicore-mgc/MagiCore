//! Supervision timeout and forbidden-child diagnostic construction.
//! Tạo chẩn đoán timeout và child bị cấm cho supervision.

use super::super::super::{ForbiddenProcess, NativeAuditContext, append_native_child_denial, tail};
use super::error::{CommandTimeoutError, ForbiddenChildError, NativeAuditOutcome};
use crate::audit::NativeChildDenial;
use std::time::Duration;

pub(in crate::run::implementation) fn forbidden_child_error(
    found: &ForbiddenProcess,
    status: std::process::ExitStatus,
    stderr: &[u8],
    stdout: &[u8],
    audit: Option<&NativeAuditContext<'_>>,
) -> ForbiddenChildError {
    let err_tail = tail(stderr);
    let out_tail = tail(stdout);
    let output_tail = if err_tail.is_empty() {
        out_tail
    } else {
        err_tail
    };
    let audit = match audit {
        Some(audit) => match append_native_child_denial(
            audit.log_path,
            NativeChildDenial {
                root_cmd: audit.root_cmd,
                root_args: audit.root_args,
                cwd: audit.cwd,
                child_pid: found.pid,
                child_name: &found.name,
                child_image: found.image_name.as_deref().unwrap_or("unavailable"),
                matched_by: found.match_source,
                matched_operation: &found.matched_operation,
            },
        ) {
            Ok(()) => NativeAuditOutcome::Recorded,
            Err(error) => NativeAuditOutcome::WriteFailed(error),
        },
        None => NativeAuditOutcome::Unavailable,
    };
    ForbiddenChildError {
        process_id: found.pid,
        name: found.name.clone(),
        image_name: found
            .image_name
            .as_deref()
            .unwrap_or("unavailable")
            .to_owned(),
        match_source: found.match_source,
        matched_token: found.matched_token.clone(),
        matched_token_index: found.matched_token_index,
        matched_operation: found.matched_operation.clone(),
        status: status.to_string(),
        audit,
        output_tail,
    }
}

pub(in crate::run::implementation) fn timeout_error(
    timeout: Duration,
    status: std::process::ExitStatus,
    stderr: &[u8],
    stdout: &[u8],
) -> CommandTimeoutError {
    let err_tail = tail(stderr);
    let out_tail = tail(stdout);
    CommandTimeoutError {
        timeout,
        status: status.to_string(),
        output_tail: if err_tail.is_empty() {
            out_tail
        } else {
            err_tail
        },
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
