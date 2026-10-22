//! Audit policy and advisory diagnostics.
//! Lỗi audit, policy và advisory.

use anyhow::Error;
use anyhow::anyhow;
use std::fmt;

/// Distinguishes audit environment failures from findings for the CLI exit contract.
/// Tách lỗi môi trường audit khỏi finding theo hợp đồng exit của CLI.
#[derive(Debug)]
pub(crate) struct AuditExitError {
    #[allow(dead_code)] // Read by the binary target; the library target has no main path.
    pub(crate) exit_code: i32,
    reason: String,
}

impl AuditExitError {
    pub(crate) fn environment(reason: &str) -> Self {
        Self {
            exit_code: 2,
            reason: reason.to_owned(),
        }
    }
}

impl fmt::Display for AuditExitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.reason)
    }
}

impl std::error::Error for AuditExitError {}

pub fn audit_strict_web_only(name: &str) -> Error {
    anyhow!(
        "--audit-strict is only implemented for the web core right now; refusing to claim policy parity for '{name}'"
    )
}

/// `--audit-strict` was passed to a command that never consults strictness.
/// Accepting it would silently ignore the flag — refuse with the supported set.
pub fn audit_strict_unsupported_command(name: &str) -> Error {
    anyhow!(
        "--audit-strict has no effect on '{name}' (strict audit applies to: audit, verify, install, add and their per-core variants)"
    )
}

pub fn audit_strict_no_web_adapter() -> Error {
    anyhow!(
        "--audit-strict requires the web core registry adapter, which is not included in this build"
    )
}

pub fn audit_parse_time_failed(name: &str, version: &str, e: &dyn std::fmt::Display) -> Error {
    anyhow!("failed to parse published time for '{name}@{version}': {e}")
}

pub fn audit_recent_blocked(name: &str, version: &str, published_at: &str) -> Error {
    anyhow!(
        "--audit-strict blocked '{name}@{version}' because it was published within the last 24 hours ({published_at})"
    )
}

pub fn audit_api_status(status: &reqwest::StatusCode) -> Error {
    anyhow!("--audit-strict advisory API returned {status}")
}

pub fn audit_advisory_blocked(pkg: &str, severity: &str, title: &str) -> Error {
    anyhow!("--audit-strict blocked '{pkg}' due to {severity} advisory: {title}")
}

pub fn audit_fix_web_only() -> Error {
    anyhow!("audit --fix is only supported for the web core")
}

pub fn audit_unknown_scanner_state(core: &str) -> anyhow::Error {
    anyhow!(
        "audit scanner status for '{core}' could not be classified — refusing to interpret the report"
    )
}

/// Environment/tooling failure (scanner unavailable under strict mode).
/// Distinct exit code 2 so CI can tell "vulnerabilities found" (1) from
/// "the audit could not run" (2) — Tech Lead contract §1.
/// Lỗi môi trường/tool (scanner unavailable ở strict mode). Exit code 2
/// riêng để CI phân biệt "có lỗ hổng" (1) với "audit không chạy được" (2).
pub fn audit_scanner_unavailable_strict(reason: &str) -> Error {
    Error::msg(AuditExitError::environment(reason))
}

pub fn audit_found_vulnerabilities(count: usize, packages: usize) -> Error {
    anyhow!("audit found {count} vulnerabilities across {packages} packages")
}

/// Unknown `--format` value for `mgc audit` (P1-B): a hard usage error —
/// never a silent fallback that would surprise CI ingest.
/// Giá trị `--format` lạ cho `mgc audit`: lỗi usage cứng — không âm thầm
/// rơi về mặc định khiến CI ingest bất ngờ.
pub fn audit_unknown_format(value: &str) -> anyhow::Error {
    anyhow!("invalid audit --format '{value}' — expected table, json, sarif, or cyclonedx")
}

pub fn web_audit_needs_context() -> Error {
    anyhow!("web audit requires project adapter context")
}
