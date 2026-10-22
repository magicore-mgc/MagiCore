//! Typed lifecycle error classification tests.
//! Test phân loại lỗi typed của lifecycle.

use super::super::error::{LifecycleError, LifecycleFailureClass};

#[test]
fn lifecycle_errors_expose_owner_failure_class() {
    let error = LifecycleError::ExternalPackageManager {
        path: "package.json".into(),
        tool: "npm".to_owned(),
    };
    assert_eq!(error.class(), LifecycleFailureClass::Policy);
}

#[test]
fn lifecycle_error_conversion_preserves_display_and_cause() {
    let error = LifecycleError::ScriptFailed {
        name: "postinstall".to_owned(),
        source: anyhow::anyhow!("root cause"),
    };
    let shared: mgc_types::error::MgError = error.into();

    assert_eq!(
        shared.to_string(),
        "lifecycle script 'postinstall' failed: root cause"
    );
    let mgc_types::error::MgError::Anyhow(source) = shared else {
        panic!("typed lifecycle cause was not preserved")
    };
    assert!(
        source
            .chain()
            .any(|cause| cause.to_string() == "root cause")
    );
}
