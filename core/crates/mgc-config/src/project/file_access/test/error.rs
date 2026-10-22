//! Public-boundary contracts for local file-access errors.
//! Contract biên public cho lỗi file access cục bộ.
use super::super::error::FileAccessError;

#[test]
fn source_errors_keep_the_original_anyhow_root_cause() {
    let original = anyhow::Error::msg("original parser cause");
    let boundary = FileAccessError::Source(original).into_anyhow();

    assert_eq!(boundary.root_cause().to_string(), "original parser cause");
}

#[test]
fn path_rejection_keeps_legacy_display_at_anyhow_boundary() {
    let error = FileAccessError::Symlink {
        label: "project config".to_owned(),
        path: std::path::PathBuf::from("mgc.toml"),
    };
    let boundary = error.into_anyhow();

    assert_eq!(
        boundary.to_string(),
        "project config 'mgc.toml' must not be a symlink"
    );
}

#[test]
fn typed_source_remains_available_before_boundary_conversion() {
    let error = FileAccessError::from(anyhow::Error::msg("typed source"));

    assert!(std::error::Error::source(&error).is_some());
}
