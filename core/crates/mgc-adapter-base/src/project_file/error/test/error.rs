//! Error conversion keeps the established message contract.
//! Kiểm tra chuyển lỗi giữ nguyên thông điệp hiện hành.

use super::{
    ProjectFileError, ProjectFileErrorKind, ProjectFileRecovery, error, error_with_source,
};
use mgc_types::MgError;
use std::error::Error;
use std::io;

#[test]
fn policy_errors_keep_message_and_recovery_classification() {
    let MgError::Anyhow(error) = error("project-file sentinel") else {
        panic!("project-file policy error was not typed");
    };
    let typed = error.downcast_ref::<ProjectFileError>().unwrap();

    assert_eq!(typed.message(), "project-file sentinel");
    assert_eq!(typed.kind(), ProjectFileErrorKind::PolicyViolation);
    assert_eq!(typed.recovery(), ProjectFileRecovery::CorrectInput);
    assert!(typed.source().is_none());
}

#[test]
fn filesystem_errors_keep_display_and_source_chain() {
    let result = error_with_source(
        io::Error::new(io::ErrorKind::NotFound, "filesystem sentinel"),
        |source| format!("read manifest: {source}"),
    );
    let MgError::Anyhow(error) = result else {
        panic!("filesystem error did not retain the typed boundary");
    };
    let typed = error.downcast_ref::<ProjectFileError>().unwrap();

    assert_eq!(typed.message(), "read manifest: filesystem sentinel");
    assert_eq!(typed.kind(), ProjectFileErrorKind::FilesystemOperation);
    assert_eq!(typed.recovery(), ProjectFileRecovery::InspectFilesystem);
    assert_eq!(typed.source().unwrap().to_string(), "filesystem sentinel");
}

#[test]
fn filesystem_error_source_is_available_through_error_trait() {
    let result = error_with_source(
        io::Error::new(io::ErrorKind::PermissionDenied, "permission sentinel"),
        |source| format!("open manifest: {source}"),
    );
    let MgError::Anyhow(error) = result else {
        panic!("filesystem error was not wrapped as anyhow");
    };
    let typed = error.downcast_ref::<ProjectFileError>().unwrap();

    assert!(Error::source(typed).is_some());
}
