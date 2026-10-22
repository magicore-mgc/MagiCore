//! Web `why` error contract tests.
//! Test contract lỗi lệnh `why` của Web.

use super::super::error::WhyError;

#[test]
fn why_errors_keep_established_user_diagnostics() {
    assert_eq!(
        WhyError::unsupported_core().to_string(),
        "why only supported for web core"
    );
    assert_eq!(
        WhyError::lock_missing().to_string(),
        "mgc.lock not found — run 'mgc install' first"
    );
    assert_eq!(
        WhyError::package_not_pinned("react").to_string(),
        "`mgc why react`: 'react' is not pinned in mgc.lock — nothing in this project can depend on it."
    );
}
