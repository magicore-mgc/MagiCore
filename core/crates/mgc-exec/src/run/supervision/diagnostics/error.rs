//! Typed supervision diagnostic errors and audit outcomes.
//! Lỗi chẩn đoán supervision và kết quả audit có kiểu.

use std::{fmt, time::Duration};

#[derive(Debug)]
pub(in crate::run::implementation::supervision) enum NativeAuditOutcome {
    Recorded,
    WriteFailed(anyhow::Error),
    Unavailable,
}

#[derive(Debug)]
pub(in crate::run::implementation) struct ForbiddenChildError {
    pub(super) process_id: u32,
    pub(super) name: String,
    pub(super) image_name: String,
    pub(super) match_source: &'static str,
    pub(super) matched_token: Option<String>,
    pub(super) matched_token_index: Option<usize>,
    pub(super) matched_operation: String,
    pub(super) status: String,
    pub(super) audit: NativeAuditOutcome,
    pub(super) output_tail: String,
}

impl fmt::Display for ForbiddenChildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let forbidden_subject = if crate::allowlist::device_control_only_tool(&self.name).is_some()
        {
            "device-control tool"
        } else {
            "forbidden package manager"
        };
        let audit_status = match &self.audit {
            NativeAuditOutcome::Recorded => "audit event recorded".to_owned(),
            NativeAuditOutcome::WriteFailed(error) => format!("audit event write failed: {error}"),
            NativeAuditOutcome::Unavailable => {
                "audit event unavailable: no log path configured".to_owned()
            }
        };
        write!(
            formatter,
            "{forbidden_subject} '{}' spawned in child process {} (image: {}; matched by: {}{}{}; invocation: {}; status after kill: {}; {})
--- tail ---
{}",
            self.name,
            self.process_id,
            self.image_name,
            self.match_source,
            self.matched_token
                .as_deref()
                .map(|token| format!(" '{token}'"))
                .unwrap_or_default(),
            self.matched_token_index
                .map(|index| format!(" at argv[{index}]"))
                .unwrap_or_default(),
            self.matched_operation,
            self.status,
            audit_status,
            self.output_tail
        )
    }
}

impl std::error::Error for ForbiddenChildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.audit {
            NativeAuditOutcome::WriteFailed(error) => Some(error.as_ref()),
            NativeAuditOutcome::Recorded | NativeAuditOutcome::Unavailable => None,
        }
    }
}

#[derive(Debug)]
pub(in crate::run::implementation) struct CommandTimeoutError {
    pub(super) timeout: Duration,
    pub(super) status: String,
    pub(super) output_tail: String,
}

impl fmt::Display for CommandTimeoutError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let budget = if self.timeout.as_secs() > 0 || self.timeout.subsec_millis() == 0 {
            format!("{}s", self.timeout.as_secs())
        } else {
            format!("{}ms", self.timeout.as_millis())
        };
        write!(
            formatter,
            "command timed out after {budget} (status after kill: {})
--- tail ---
{}",
            self.status, self.output_tail
        )
    }
}

impl std::error::Error for CommandTimeoutError {}
