//! Unsupported error constructor contract tests.
//! Test hợp đồng hàm tạo lỗi unsupported.

use crate::capabilities::test_support::assert_unsupported;
use crate::capabilities::unsupported_capability;

#[test]
fn constructor_preserves_error_fields_and_display_contract() {
    let guidance = "enable a native provider";
    let error = unsupported_capability("web", "resolve", guidance);
    assert_eq!(
        error.to_string(),
        "web core does not support 'resolve' yet: enable a native provider"
    );
    assert_unsupported(error, "web", "resolve", guidance);
}
