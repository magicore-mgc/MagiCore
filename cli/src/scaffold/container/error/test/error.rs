//! Exact-message and typed-downcast tests for container errors.
//! Test nội dung lỗi chính xác và downcast typed cho container.

use super::*;

#[test]
fn split_web_runtime_preserves_message_and_type() {
    let error = web_container_runtime_split_unsupported();
    assert_eq!(
        error.to_string(),
        concat!(
            "container scaffolding currently requires one runtime per web project; ",
            "split fullstack/monorepo containers are not implemented"
        )
    );
    assert!(matches!(
        error.downcast_ref::<ContainerScaffoldError>(),
        Some(ContainerScaffoldError::WebRuntimeSplitUnsupported)
    ));
}

#[test]
fn unsupported_profile_preserves_message_and_type() {
    let error = container_profile_unsupported("web", "astro", "runtime image");
    assert_eq!(
        error.to_string(),
        concat!(
            "container scaffolding has no MGC lifecycle profile for core 'web', ",
            "framework 'astro', runtime 'runtime image'"
        )
    );
    assert!(matches!(
        error.downcast_ref::<ContainerScaffoldError>(),
        Some(ContainerScaffoldError::ProfileUnsupported { .. })
    ));
}

#[test]
fn missing_manifest_preserves_message_and_type() {
    let error = container_manifest_required("ai", "python-agent", "pyproject.toml");
    assert_eq!(
        error.to_string(),
        concat!(
            "Docker scaffolding for core 'ai' framework 'python-agent' ",
            "requires the supported project manifest 'pyproject.toml'"
        )
    );
    assert!(matches!(
        error.downcast_ref::<ContainerScaffoldError>(),
        Some(ContainerScaffoldError::ManifestRequired { .. })
    ));
}
