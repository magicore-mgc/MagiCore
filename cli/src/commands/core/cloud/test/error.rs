//! Cloud adapter error contract tests.
//! Test contract lỗi adapter Cloud.

use super::super::error::CloudAdapterError;
use std::error::Error as StdError;

#[test]
fn cloud_adapter_error_keeps_the_original_diagnostic_and_cause() {
    let source = anyhow::anyhow!("cloud context rejected");
    let error = CloudAdapterError::project_context(source);

    assert_eq!(error.to_string(), "cloud context rejected");
    assert_eq!(
        StdError::source(&error)
            .expect("the adapter boundary retains its cause")
            .to_string(),
        "cloud context rejected"
    );
}
