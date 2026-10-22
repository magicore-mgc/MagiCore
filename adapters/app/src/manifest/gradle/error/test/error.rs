//! Tests for the local-to-public Gradle error boundary.
//! Test ranh giới chuyển đổi lỗi Gradle cục bộ sang lỗi công khai.

use super::super::GradleManifestError;
use mgc_types::MgError;

#[test]
fn other_error_keeps_its_display_and_public_variant() {
    let local = GradleManifestError::Other("unsupported libs.versions.toml shape".to_string());
    assert_eq!(local.to_string(), "unsupported libs.versions.toml shape");
    assert!(matches!(
        local.into_mg_error(),
        MgError::Other(message) if message == "unsupported libs.versions.toml shape"
    ));
}

#[test]
fn unsupported_error_keeps_public_fields_and_display() {
    let local = GradleManifestError::Unsupported {
        core: "app",
        capability: "parse-gradle-manifest",
        guidance: "use version catalog".to_string(),
    };
    assert_eq!(
        local.to_string(),
        "app core does not support 'parse-gradle-manifest' yet: use version catalog"
    );
    assert!(matches!(
        local.into_mg_error(),
        MgError::Unsupported {
            core: "app",
            capability: "parse-gradle-manifest",
            guidance
        } if guidance == "use version catalog"
    ));
}

#[test]
fn source_io_error_is_restored_without_losing_its_type() {
    let source = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
    let local = GradleManifestError::from(MgError::Io(source));
    assert!(matches!(
        local.into_mg_error(),
        MgError::Io(error) if error.kind() == std::io::ErrorKind::PermissionDenied
    ));
}
