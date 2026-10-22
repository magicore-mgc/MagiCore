//! Typed Gradle lock failures mapped to the established shared error API.
//! Lỗi Gradle lock có kiểu, ánh xạ về API lỗi dùng chung hiện hành.

use mgc_types::MgError;

#[derive(Debug, thiserror::Error)]
pub(super) enum GradleLockError {
    #[error(
        "gradle.lockfile line {line} is not group:artifact:version[:configuration]: '{input}' (fail-closed)"
    )]
    MalformedLine { line: usize, input: String },
    #[error(
        "{coordinate} is pinned to both {first_version} and {second_version} in gradle.lockfile — refusing to guess (fail-closed)"
    )]
    ConflictingVersions {
        coordinate: String,
        first_version: String,
        second_version: String,
    },
}

impl From<GradleLockError> for MgError {
    fn from(error: GradleLockError) -> Self {
        Self::Other(error.to_string())
    }
}

#[cfg(test)]
#[path = "test/gradle_error.rs"]
mod tests;
