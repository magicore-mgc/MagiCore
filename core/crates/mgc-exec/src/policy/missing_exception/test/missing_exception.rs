use crate::policy::{DenyCode, PolicyDecision, missing_exception};

// Missing-exception helper reports the row-5 code.
// (Helper thiếu exception báo mã hàng 5.)
#[test]
fn missing_exception_helper() {
    assert_eq!(
        missing_exception(),
        PolicyDecision::Deny {
            code: DenyCode::MissingException
        }
    );
}
