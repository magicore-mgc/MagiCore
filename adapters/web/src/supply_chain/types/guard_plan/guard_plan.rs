//! Pure policy plan for the existing supply-chain guard sequence.
//! Kế hoạch policy thuần cho thứ tự guard chuỗi cung ứng hiện hữu.

/// Immutable decisions derived from the current environment and release policy.
/// Quyết định bất biến suy ra từ môi trường và policy tuổi release hiện tại.
pub(crate) struct GuardPlan {
    pub(crate) enforce_publish_age: bool,
    pub(crate) enforce_no_downgrade: bool,
    pub(crate) allow_untrusted: bool,
    pub(crate) min_age_secs: i64,
    pub(crate) allow_missing_time: bool,
}

impl GuardPlan {
    /// The age guard runs whenever a policy age is present — the default
    /// quarantine is applied upstream so `min_age_secs` is `Some(24h)` on a
    /// first resolution. Explicit env flags stay as additional requests.
    /// (Guard tuổi chạy khi có policy age — quarantine mặc định được áp ở
    /// tầng trên nên resolve lần đầu có `Some(24h)`.)
    pub(crate) fn from_settings(
        age_policy: super::super::AgePolicySettings,
        explicit_age_block: bool,
        audit_strict: bool,
        allow_untrusted: bool,
        default_quarantine_secs: i64,
    ) -> Self {
        let age_gate_requested =
            explicit_age_block || audit_strict || age_policy.min_age_secs.is_some();
        let min_age_secs = age_policy.min_age_secs.unwrap_or(default_quarantine_secs);

        Self {
            enforce_publish_age: age_gate_requested && !allow_untrusted,
            enforce_no_downgrade: !allow_untrusted,
            allow_untrusted,
            min_age_secs,
            allow_missing_time: age_policy.allow_missing_time,
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
