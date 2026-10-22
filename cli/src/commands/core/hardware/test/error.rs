//! Hardware materialization error contract tests.
//! Test contract lỗi materialize Hardware.

use super::super::error::HardwareMaterializeError;
use super::super::implementation::require_layer_available;
use crate::scaffold::resolver::ScaffoldResolveError;
use std::error::Error as StdError;

#[test]
fn missing_layer_keeps_the_existing_diagnostic() {
    let error = HardwareMaterializeError::missing_layer("custom-fw");

    assert_eq!(
        error.to_string(),
        "Required hardware template layer 'hardware/custom-fw' missing - no fallback available"
    );
    assert!(StdError::source(&error).is_none());
}

#[test]
fn scaffold_failure_retains_its_source() {
    let error = HardwareMaterializeError::scaffold(anyhow::anyhow!("processor rejected config"));

    assert_eq!(error.to_string(), "processor rejected config");
    assert_eq!(
        StdError::source(&error)
            .expect("scaffold cause")
            .to_string(),
        "processor rejected config"
    );
}

#[test]
fn resolver_failure_keeps_its_diagnostic_and_typed_source() {
    let result = require_layer_available(
        "custom-fw",
        Err(ScaffoldResolveError::Other(
            "registry fetch failed".to_string(),
        )),
    );
    let error = result.expect_err("resolver failures must not be mislabeled as missing layers");

    assert_eq!(
        error.to_string(),
        "Scaffold resolve failed: registry fetch failed"
    );
    let source = StdError::source(&error).expect("resolver failure remains in the source chain");
    assert_eq!(
        source.to_string(),
        "Scaffold resolve failed: registry fetch failed"
    );
    assert!(source.downcast_ref::<ScaffoldResolveError>().is_some());
}

#[test]
fn target_inspection_failure_retains_its_io_source() {
    let source = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "metadata denied");
    let error = HardwareMaterializeError::target_inspection(
        std::path::PathBuf::from("project/bench"),
        source,
    );

    assert!(
        error
            .to_string()
            .contains("Cannot inspect Hardware package target")
    );
    assert_eq!(
        StdError::source(&error)
            .expect("target inspection keeps its I/O source")
            .to_string(),
        "metadata denied"
    );
}

#[test]
fn unavailable_status_keeps_the_no_fallback_diagnostic() {
    let result = require_layer_available(
        "custom-fw",
        Ok(
            crate::scaffold::resolver::ScaffoldResolveStatus::OptionalMissing {
                layer: "hardware/custom-fw".to_string(),
                reason: "not present".to_string(),
            },
        ),
    );
    let error = result.expect_err("optional absence is not usable by Hardware materialization");

    assert_eq!(
        error.to_string(),
        "Required hardware template layer 'hardware/custom-fw' missing - no fallback available"
    );
}
