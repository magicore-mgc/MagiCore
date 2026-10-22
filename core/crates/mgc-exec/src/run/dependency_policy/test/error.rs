//! Typed dependency denial error tests.
//! Test lỗi từ chối dependency typed.

use super::*;
use std::error::Error;

#[test]
fn dependency_denial_is_typed_and_has_no_secondary_source() {
    let error = DependencyResolutionError::new("Cargo mutation denied".to_owned());
    assert_eq!(
        error.to_string(),
        "external dependency resolution blocked: Cargo mutation denied"
    );
    assert!(error.source().is_none());
}
