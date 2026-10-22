//! Verifies health severity precedence and empty-report status.
//! Kiểm tra thứ tự ưu tiên severity và trạng thái report rỗng.

use super::super::classify;
use crate::commands::doctor::DiagnosticIssue;

fn issue(severity: &'static str) -> DiagnosticIssue {
    DiagnosticIssue {
        code: "TEST",
        severity,
        message: String::new(),
        fix_command: None,
    }
}

#[test]
fn critical_precedes_warning_and_empty_is_healthy() {
    assert_eq!(classify(&[]), "HEALTHY");
    assert_eq!(classify(&[issue("WARNING")]), "DEGRADED");
    assert_eq!(
        classify(&[issue("WARNING"), issue("CRITICAL")]),
        "UNHEALTHY"
    );
}
