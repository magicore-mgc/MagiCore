//! Resolve strict mode and enforce the audit exit contract.
//! Xác định strict mode và giữ hợp đồng exit của audit.

use anyhow::Result;
use mgc_types::adapter::AuditReport;

/// Exit decisions only — no rendering (P0-1: --fix renders first,
/// mutates, re-audits, and only then calls this on the POST report).
/// (Chỉ quyết exit — không in gì.)
pub(crate) fn enforce_audit_exit(
    core: &str,
    report: &AuditReport,
    strict: StrictMode,
) -> Result<()> {
    if report.vulnerability_count > 0 {
        return Err(crate::error::audit_found_vulnerabilities(
            report.vulnerability_count,
            report.packages_audited,
        ));
    }
    if !report.scanner_available() {
        if strict.enabled() {
            return Err(crate::error::audit_scanner_unavailable_strict(&format!(
                "{core} audit did not complete"
            )));
        }
        return Ok(());
    }
    Ok(())
}

/// Strict audit mode flag — `MGC_AUDIT_STRICT` env (set by `--audit-strict`).
/// Escape hatch contract (RULE §11): default open locally, closed in CI.
/// `MGC_AUDIT_STRICT=1` forces strict anywhere; when unset, CI environments
/// (`CI=true|1`, as set by GitHub/GitLab/Circle) default to strict — an
/// UNVERIFIED audit must never pass a pipeline silently (Tech Lead P0-3).
/// Cờ strict — env `MGC_AUDIT_STRICT` (bật qua `--audit-strict`).
/// Hợp đồng escape hatch (RULE §11): mặc định mở locally, đóng trong CI —
/// CI=true tự strict, UNVERIFIED không thể lọt pipeline một cách im lặng.
#[derive(Debug, Clone, Copy)]
pub(crate) struct StrictMode(bool);

impl StrictMode {
    pub(crate) fn required() -> Self {
        Self(true)
    }

    pub(crate) fn from_env() -> Self {
        // Explicit beats implicit: a SET MGC_AUDIT_STRICT always decides
        // ("1"/"true" strict, anything else — including "0" — open), so
        // tests and operators can force the open lane anywhere. Only an
        // UNSET variable falls back to the CI-runner default.
        // (Tường minh thắng ngầm định: SET là quyết, chỉ UNSET mới theo CI.)
        if let Ok(v) = std::env::var("MGC_AUDIT_STRICT") {
            return Self(v == "1" || v.to_lowercase() == "true");
        }
        // CI runners export CI=true — strict is the safe default there.
        // CI runner export CI=true — strict là mặc định an toàn ở đó.
        let in_ci = std::env::var("CI")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);
        Self(in_ci)
    }

    fn enabled(self) -> bool {
        self.0
    }
}
