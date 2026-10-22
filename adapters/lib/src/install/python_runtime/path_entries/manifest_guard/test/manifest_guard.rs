//! Python foreign-manifest fail-closed tests.
//! Test từ chối manifest ngoài phạm vi Python.

use super::support::*;

#[test]
fn python_runtime_rejects_external_dependency_manifests_instead_of_ambient_fallback() {
    for manifest in ["requirements.txt", "uv.lock", "Pipfile", "setup.py"] {
        let project = tempfile::tempdir().unwrap();
        std::fs::write(
            project.path().join(manifest),
            "# external dependency metadata\n",
        )
        .unwrap();

        let error = native_python_path_entries(project.path()).unwrap_err();

        assert!(
            error
                .to_string()
                .contains(manifest.to_ascii_lowercase().as_str())
        );
        assert!(error.to_string().contains("ambient Python packages"));
    }
}
