//! Typed errors for MGC-owned container scaffold profiles.
//! Lỗi typed cho profile scaffold container do MGC quản lý.

use anyhow::Error;

#[derive(Debug, thiserror::Error)]
enum ContainerScaffoldError {
    /// A split Web runtime cannot fit the existing single-image profile.
    /// Runtime Web tách không thể dùng profile một image hiện có.
    #[error(
        "container scaffolding currently requires one runtime per web project; \
         split fullstack/monorepo containers are not implemented"
    )]
    WebRuntimeSplitUnsupported,

    /// No container lifecycle profile exists for the selected core/runtime.
    /// Core/runtime đã chọn chưa có profile lifecycle container.
    #[error(
        "container scaffolding has no MGC lifecycle profile for core '{core}', \
         framework '{framework}', runtime '{runtime}'"
    )]
    ProfileUnsupported {
        core: String,
        framework: String,
        runtime: String,
    },

    /// The selected profile requires a native project manifest.
    /// Profile đã chọn cần manifest project native.
    #[error(
        "Docker scaffolding for core '{core}' framework '{framework}' \
         requires the supported project manifest '{manifest}'"
    )]
    ManifestRequired {
        core: String,
        framework: String,
        manifest: String,
    },
}

/// Return the stable diagnostic for an unsupported split Web runtime.
/// Trả diagnostic ổn định khi runtime Web bị tách không được hỗ trợ.
pub(in crate::scaffold::container) fn web_container_runtime_split_unsupported() -> Error {
    Error::new(ContainerScaffoldError::WebRuntimeSplitUnsupported)
}

/// Return the stable diagnostic when no MGC container lifecycle exists.
/// Trả diagnostic ổn định khi chưa có lifecycle container MGC.
pub(in crate::scaffold::container) fn container_profile_unsupported(
    core: &str,
    framework: &str,
    runtime: &str,
) -> Error {
    Error::new(ContainerScaffoldError::ProfileUnsupported {
        core: core.to_owned(),
        framework: framework.to_owned(),
        runtime: runtime.to_owned(),
    })
}

/// Return the stable diagnostic when a required native manifest is missing.
/// Trả diagnostic ổn định khi thiếu manifest native bắt buộc.
pub(in crate::scaffold::container) fn container_manifest_required(
    core: &str,
    framework: &str,
    manifest: &str,
) -> Error {
    Error::new(ContainerScaffoldError::ManifestRequired {
        core: core.to_owned(),
        framework: framework.to_owned(),
        manifest: manifest.to_owned(),
    })
}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
