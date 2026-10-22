#![allow(clippy::unwrap_used)]
//! SPDX error boundary tests — Test boundary lỗi SPDX

use super::super::super::*;
use crate::SbomError;
use std::error::Error;

#[test]
fn maps_invalid_names_and_graph_failures_to_existing_public_messages() {
    let cases = [
        (
            SpdxError::EmptyDocumentName,
            "SPDX document name must not be empty",
        ),
        (
            SpdxError::EmptyPackageName,
            "SPDX package name must not be empty",
        ),
        (
            SpdxError::DuplicatePackageIdentity("pkg:one".into()),
            "duplicate SPDX package identity: pkg:one",
        ),
        (
            SpdxError::PackageIdentityDisappeared("pkg:two".into()),
            "SPDX package identity disappeared: pkg:two",
        ),
        (
            SpdxError::DependencySourceMissing("pkg:three".into()),
            "SPDX dependency source is missing from package graph: pkg:three",
        ),
        (
            SpdxError::DependencyTargetMissing("pkg:four".into()),
            "SPDX dependency target is missing from package graph: pkg:four",
        ),
    ];

    for (error, expected) in cases {
        let public = error.into_public();
        assert!(matches!(&public, SbomError::InvalidLockfile(_)));
        assert_eq!(public.to_string(), format!("Invalid lockfile: {expected}"));
    }
}

#[test]
fn maps_json_failure_without_dropping_its_source() {
    let source = serde_json::from_str::<serde_json::Value>("{").unwrap_err();
    let public = SpdxError::Serialization(source).into_public();

    assert!(matches!(&public, SbomError::SerializationError(_)));
    assert!(public.source().is_some());
}
