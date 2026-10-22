//! Tests the shared v3 serializer's source-completeness contract.
//! Kiểm tra contract completeness nguồn của serializer v3 dùng chung.

use super::super::serialize_lockfile;
use crate::{
    EcosystemTag, Lockfile, LockfileError, Package, Provenance, SOURCE_KIND_DELEGATED_TOOL,
};

fn classified_package() -> Package {
    let mut package = Package::new(
        "serde".to_string(),
        "1.0.0".to_string(),
        "https://example.invalid/serde".to_string(),
        "blake3-test".to_string(),
    );
    package.ecosystem = EcosystemTag::Rust;
    package
}

#[test]
fn classified_package_without_source_is_rejected_before_serialization() {
    let mut lock = Lockfile::new();
    lock.add_package(classified_package());

    assert!(matches!(
        serialize_lockfile(&lock),
        Err(LockfileError::IncompleteProvenance(message))
            if message.contains("serde@1.0.0")
    ));
}

#[test]
fn empty_registry_is_rejected_before_serialization() {
    let mut lock = Lockfile::new();
    let mut package = classified_package();
    package.registry = Some(" \t ".to_string());
    lock.add_package(package);

    assert!(matches!(
        serialize_lockfile(&lock),
        Err(LockfileError::IncompleteProvenance(message))
            if message.contains("serde@1.0.0")
    ));
}

#[test]
fn registry_source_allows_v3_serialization() {
    let mut lock = Lockfile::new();
    let mut package = classified_package();
    package.registry = Some("crates://sparse.crates.io".to_string());
    lock.add_package(package);

    assert!(serialize_lockfile(&lock).is_ok());
}

#[test]
fn delegated_tool_provenance_allows_v3_serialization() {
    let mut lock = Lockfile::new();
    let mut package = classified_package();
    package.provenance = Some(Provenance {
        source_kind: SOURCE_KIND_DELEGATED_TOOL.to_string(),
        tool: Some("cargo 1.85".to_string()),
        imported_from: None,
    });
    lock.add_package(package);

    assert!(serialize_lockfile(&lock).is_ok());
}

#[test]
fn imported_lockfile_provenance_allows_v3_serialization() {
    let mut lock = Lockfile::new();
    let mut package = classified_package();
    package.provenance = Some(Provenance {
        source_kind: crate::SOURCE_KIND_REGISTRY_IMPORT.to_string(),
        tool: None,
        imported_from: Some("package-lock.json".to_string()),
    });
    lock.add_package(package);

    assert!(serialize_lockfile(&lock).is_ok());
}

#[test]
fn unnamed_delegated_tool_is_rejected_before_serialization() {
    let mut lock = Lockfile::new();
    let mut package = classified_package();
    package.provenance = Some(Provenance {
        source_kind: SOURCE_KIND_DELEGATED_TOOL.to_string(),
        tool: None,
        imported_from: None,
    });
    lock.add_package(package);

    assert!(matches!(
        serialize_lockfile(&lock),
        Err(LockfileError::IncompleteProvenance(message))
            if message.contains("serde@1.0.0")
    ));
}

#[test]
fn unclassified_legacy_rows_remain_serializable() {
    let mut lock = Lockfile::new();
    lock.add_package(Package::new(
        "legacy-pin".to_string(),
        "1.0.0".to_string(),
        String::new(),
        String::new(),
    ));

    assert!(serialize_lockfile(&lock).is_ok());
}
