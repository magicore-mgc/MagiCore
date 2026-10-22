#![allow(clippy::unwrap_used)]
//! Document-name validation cases — Trường hợp xác thực tên tài liệu

use super::super::super::*;
use crate::spdx::error::SpdxError;

#[test]
fn rejects_empty_and_whitespace_only_names() {
    assert!(matches!(
        validate_document_name(""),
        Err(SpdxError::EmptyDocumentName)
    ));
    assert!(matches!(
        validate_document_name(" \t\n"),
        Err(SpdxError::EmptyDocumentName)
    ));
}

#[test]
fn accepts_non_empty_document_names() {
    assert!(validate_document_name("project").is_ok());
}
