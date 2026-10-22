//! Manifest error taxonomy and source-chain tests.
//! Test taxonomy và source chain lỗi manifest.

use super::super::ManifestError;
use mgc_types::MgError;
use std::path::PathBuf;

#[test]
fn workspace_replacement_keeps_structured_unsupported_fields() {
    let error = MgError::from(ManifestError::WorkspaceDependencyReplacement {
        name: "@core/shared".to_string(),
    });

    match error {
        MgError::Unsupported {
            core,
            capability,
            guidance,
        } => {
            assert_eq!(core, "web");
            assert_eq!(capability, "replace workspace dependency source");
            assert_eq!(
                guidance,
                "dependency '@core/shared' is currently workspace-owned; MagiCore refuses to silently replace its source with a registry pin"
            );
        }
        other => panic!("expected structured Unsupported, got {other:?}"),
    }
}

#[test]
fn io_failure_keeps_typed_manifest_error_and_source() {
    let error = ManifestError::InspectAtomicWriteDestination {
        path: PathBuf::from("package.json"),
        source: std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied"),
    };
    let mapped = MgError::from(error);

    match mapped {
        MgError::Anyhow(error) => {
            assert!(error.downcast_ref::<ManifestError>().is_some());
            assert!(error.source().is_some());
            assert_eq!(
                error.to_string(),
                "inspect atomic-write destination package.json: denied"
            );
        }
        other => panic!("expected retained typed error, got {other:?}"),
    }
}

#[test]
fn policy_error_keeps_its_legacy_display_text() {
    let mapped = MgError::from(ManifestError::MissingPackageJson {
        project_root: PathBuf::from("project"),
    });

    match mapped {
        MgError::Anyhow(error) => assert_eq!(
            error.to_string(),
            "No package.json in 'project'. Run 'mgc init --template web' first."
        ),
        other => panic!("expected typed manifest error, got {other:?}"),
    }
}

#[test]
fn atomic_path_error_keeps_its_legacy_display_text() {
    let mapped = MgError::from(ManifestError::UnsafeAtomicWriteDestination {
        path: PathBuf::from("cache/state.json"),
    });

    match mapped {
        MgError::Anyhow(error) => assert_eq!(
            error.to_string(),
            "refusing to atomically write non-regular or linked path: cache/state.json"
        ),
        other => panic!("expected typed manifest error, got {other:?}"),
    }
}
