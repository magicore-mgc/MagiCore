//! Derives the stable health label from diagnostic severity.
//! Tính health label ổn định từ severity của diagnostic.

use crate::commands::doctor::DiagnosticIssue;

pub(in crate::commands::doctor::analysis) fn classify(issues: &[DiagnosticIssue]) -> &'static str {
    if issues.iter().any(|issue| issue.severity == "CRITICAL") {
        "UNHEALTHY"
    } else if !issues.is_empty() {
        "DEGRADED"
    } else {
        "HEALTHY"
    }
}
