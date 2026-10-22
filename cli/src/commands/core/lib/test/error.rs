//! Lib adapter error contract tests.
//! Test contract lỗi adapter Lib.

use super::super::error::LibAdapterError;
use std::error::Error as StdError;

#[test]
fn lib_adapter_error_keeps_the_original_diagnostic_and_cause() {
    let source = anyhow::anyhow!("registry configuration rejected");
    let error = LibAdapterError::registry_adapter(source);

    assert_eq!(error.to_string(), "registry configuration rejected");
    assert_eq!(
        StdError::source(&error)
            .expect("the adapter boundary retains its cause")
            .to_string(),
        "registry configuration rejected"
    );
}
