//! SPDX document-name validation — Xác thực tên tài liệu SPDX

use super::super::error::{SpdxError, SpdxResult};

pub(in crate::spdx) fn validate_document_name(document_name: &str) -> SpdxResult<()> {
    if document_name.trim().is_empty() {
        return Err(SpdxError::EmptyDocumentName);
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
