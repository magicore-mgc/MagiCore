//! Import error contracts — kiểm tra contract lỗi import.
#![allow(clippy::unwrap_used)]

use super::super::ImportError;
use std::error::Error as _;
use std::path::PathBuf;

#[test]
fn artifact_read_errors_keep_path_message_and_lockfile_source() {
    let source = mgc_lockfile::LockfileError::WriteFailed("denied".to_string());
    let lock = ImportError::LockfileRead {
        path: PathBuf::from("mgc.lock"),
        source,
    };
    assert_eq!(
        lock.to_string(),
        "cannot safely read 'mgc.lock': lock write failed: denied"
    );
    assert!(lock.source().is_some());

    let source = mgc_lockfile::LockfileError::WriteFailed("denied".to_string());
    let signature = ImportError::SignatureRead {
        path: PathBuf::from("mgc.lock.sig"),
        source,
    };
    assert_eq!(
        signature.to_string(),
        "cannot safely read 'mgc.lock.sig': lock write failed: denied"
    );
    assert!(signature.source().is_some());
}

#[test]
fn project_config_error_keeps_message_and_source() {
    let error = ImportError::ProjectConfig(anyhow::anyhow!("config parse failed"));
    assert_eq!(error.to_string(), "config parse failed");
    assert!(error.source().is_some());
}

#[test]
fn unknown_owner_error_is_stable_and_has_no_fake_source() {
    let error = ImportError::UnsupportedCoreOwner {
        core: "unknown".to_string(),
    };
    assert_eq!(
        error.to_string(),
        "refusing to import lock for unknown core owner 'unknown'"
    );
    assert!(error.source().is_none());
}
