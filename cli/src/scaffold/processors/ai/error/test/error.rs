//! Tests that typed wrapping preserves the original write error.
//! Test wrapper typed bảo toàn lỗi ghi gốc.

use std::error::Error as _;

#[test]
fn wrapper_keeps_the_source_error() {
    let source = anyhow::anyhow!("writer failed");
    let wrapped = super::super::AiScaffoldError::from(source);
    assert_eq!(wrapped.to_string(), "writer failed");
    assert!(wrapped.source().is_some());
}
