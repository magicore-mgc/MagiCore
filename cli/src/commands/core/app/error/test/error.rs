//! App core error contracts.
//! Contract lỗi của App core.

use crate::commands::core::app::error::AppCoreError;

#[test]
fn app_error_display_preserves_existing_user_diagnostics() {
    assert_eq!(
        AppCoreError::project_not_detected().to_string(),
        "Cannot detect an app project here (missing mgc.toml [app] language / pubspec.yaml / build.gradle/.kts / Package.swift)."
    );
    assert_eq!(
        AppCoreError::native_lifecycle_unavailable("pub", "install").to_string(),
        "`app` install for `pub` is unavailable: MagiCore does not yet own the complete native dependency lifecycle for this lane. No external package manager was invoked; use a supported native lane or wait for native resolver, lock, fetch, verify, store, and materializer support."
    );
}

#[test]
fn app_manifest_hint_error_retains_language_and_manifest_context() {
    let error = AppCoreError::manifest_hint("update", "Flutter", "pubspec.yaml");
    assert_eq!(
        error.to_string(),
        "'update' for \"Flutter\" has no CLI passthrough — edit pubspec.yaml then run `mgc install`."
    );
}
