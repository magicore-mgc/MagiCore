use super::super::*;

#[test]
fn validates_existing_layer_paths_without_fetching_missing_layers() {
    use crate::scaffold::resolver::ScaffoldResolveError;
    use std::fs;

    let root = tempfile::tempdir().unwrap();
    let corrupt_layer = root.path().join("web/shared/partials/base");
    fs::create_dir_all(corrupt_layer.parent().unwrap()).unwrap();
    fs::write(&corrupt_layer, b"not-a-template-directory").unwrap();

    let error = validate_cached_layer_path_in("web/shared/partials/base", root.path()).unwrap_err();

    assert!(matches!(error, ScaffoldResolveError::Other(_)));
    assert!(error.to_string().contains("refusing unsafe"));
    assert!(!error.is_availability_failure());
}

#[test]
fn only_availability_errors_allow_scaffold_fallback() {
    for status in [401, 403, 404, 410, 429, 503] {
        let error = template_resolution_error(
            crate::error::template_download_status(status),
            "ai/probe",
            "ai",
            "probe",
            "latest",
        );
        assert_eq!(
            error.is_availability_failure(),
            matches!(status, 404 | 410 | 429 | 503)
        );
    }
    let error = template_resolution_error(
        crate::error::template_integrity_mismatch(),
        "lib/probe",
        "lib",
        "probe",
        "latest",
    );
    assert!(!error.is_availability_failure());
}
