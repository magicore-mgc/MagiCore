#![allow(clippy::unwrap_used)]
//! Public-boundary contracts for OCI blob errors.
//! Hợp đồng biên công khai cho lỗi OCI blob.

use super::OciBlobError;

#[test]
fn source_errors_keep_the_original_anyhow_root_cause() {
    let source = anyhow::Error::msg("original registry source");
    let public_error = OciBlobError::Source(source).into_anyhow();

    assert_eq!(
        public_error.root_cause().to_string(),
        "original registry source"
    );
}

#[test]
fn domain_error_keeps_the_legacy_digest_mismatch_message() {
    let error = OciBlobError::DigestMismatch {
        declared: "sha256:expected".to_owned(),
        actual: "sha256:actual".to_owned(),
    };
    let expected = concat!(
        "oci digest mismatch: declared 'sha256:expected' ",
        "but bytes hash to 'sha256:actual'"
    );

    assert_eq!(error.to_string(), expected);
    assert_eq!(error.into_anyhow().to_string(), expected);
}

#[test]
fn domain_errors_keep_legacy_size_and_integrity_messages() {
    let size_error = OciBlobError::SizeLimitExceeded;
    let integrity_error = OciBlobError::IntegrityMismatch {
        digest: "sha256:expected".to_owned(),
        repo: "ai/models".to_owned(),
        actual: "sha256:actual".to_owned(),
    };

    assert_eq!(
        size_error.to_string(),
        "OCI blob exceeds configured maximum size"
    );
    assert_eq!(
        integrity_error.to_string(),
        concat!(
            "oci blob integrity mismatch for 'sha256:expected' in repo 'ai/models': ",
            "bytes hash to 'sha256:actual'"
        )
    );
}

#[test]
fn filesystem_error_keeps_its_original_io_source() {
    let storage_io = OciBlobError::from(std::io::Error::other("temporary storage failure"));

    assert_eq!(
        storage_io.into_anyhow().root_cause().to_string(),
        "temporary storage failure"
    );
}
