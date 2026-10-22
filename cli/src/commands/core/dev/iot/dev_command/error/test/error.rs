//! Tests for IoT-specific error messages.

use super::super::IotDevError;

#[test]
fn unsupported_framework_names_the_framework() {
    let error = IotDevError::UnsupportedFramework {
        framework: "custom-iot".to_string(),
    };

    assert!(error.to_string().contains("custom-iot"));
}
