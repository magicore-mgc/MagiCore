//! Audit policy and advisory diagnostics.
//! Lỗi audit, policy và advisory.

mod error;

#[allow(unused_imports)] // Also consumed by the CLI binary's exit-code path.
pub(crate) use error::AuditExitError;
pub use error::{
    audit_advisory_blocked, audit_api_status, audit_fix_web_only, audit_found_vulnerabilities,
    audit_parse_time_failed, audit_recent_blocked, audit_scanner_unavailable_strict,
    audit_strict_no_web_adapter, audit_strict_unsupported_command, audit_strict_web_only,
    audit_unknown_format, audit_unknown_scanner_state, web_audit_needs_context,
};
