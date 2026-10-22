//! Checks that legacy loading cannot erase v4 package identity.
//! Kiểm tra loader legacy không làm mất identity package v4.

use super::super::require_legacy_document;
use crate::{LockDocument, Lockfile, LockfileError};

#[test]
fn legacy_model_accepts_legacy_documents() {
    let loaded = require_legacy_document(LockDocument::Legacy(Lockfile::new()))
        .expect("test operation must succeed");
    assert_eq!(loaded.version, crate::LOCKFILE_SCHEMA_VERSION);
}

#[test]
fn legacy_model_rejects_v4_documents() {
    let document = LockDocument::V4(crate::canonical::LockfileV4::new("mgc/test"));
    assert!(matches!(
        require_legacy_document(document),
        Err(LockfileError::ParseError(message)) if message.contains("lossless LockDocument")
    ));
}
