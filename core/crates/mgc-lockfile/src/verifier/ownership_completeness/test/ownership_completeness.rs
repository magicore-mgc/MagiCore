//! Checks typed provenance failures and the legacy ecosystem exemption.
//! Kiểm tra lỗi provenance có kiểu và miễn trừ hệ sinh thái legacy.

use super::super::verify_ownership_completeness;
use crate::{EcosystemTag, Lockfile, LockfileError, Package};

#[test]
fn classified_package_without_source_gets_stable_error() {
    let mut lock = Lockfile::new();
    let mut package = Package::new(
        "serde".to_string(),
        "1.0.0".to_string(),
        "".into(),
        "".into(),
    );
    package.ecosystem = EcosystemTag::Rust;
    lock.add_package(package);
    assert!(matches!(
        verify_ownership_completeness(&lock),
        Err(LockfileError::IncompleteProvenance(message))
            if message == "package 'serde@1.0.0' (ecosystem rust) has no complete source identity (registry, imported lockfile, or named delegated tool)"
    ));
}

#[test]
fn other_ecosystem_remains_exempt() {
    let mut lock = Lockfile::new();
    lock.add_package(Package::new(
        "legacy".into(),
        "1.0.0".into(),
        "".into(),
        "".into(),
    ));
    assert!(verify_ownership_completeness(&lock).is_ok());
}

#[test]
fn empty_registry_does_not_count_as_package_source() {
    for registry in ["", " \t "] {
        let mut lock = Lockfile::new();
        let mut package = Package::new("serde".into(), "1.0.0".into(), "".into(), "".into());
        package.ecosystem = EcosystemTag::Rust;
        package.registry = Some(registry.to_string());
        lock.add_package(package);

        assert!(matches!(
            verify_ownership_completeness(&lock),
            Err(LockfileError::IncompleteProvenance(_))
        ));
    }
}

#[test]
fn delegated_provenance_requires_a_nonempty_tool_identifier() {
    for tool in [None, Some(""), Some(" \t ")] {
        let mut lock = Lockfile::new();
        let mut package = Package::new("serde".into(), "1.0.0".into(), "".into(), "".into());
        package.ecosystem = EcosystemTag::Rust;
        package.provenance = Some(crate::Provenance {
            source_kind: crate::SOURCE_KIND_DELEGATED_TOOL.to_string(),
            tool: tool.map(|value| value.to_string()),
            imported_from: None,
        });
        lock.add_package(package);

        assert!(matches!(
            verify_ownership_completeness(&lock),
            Err(LockfileError::IncompleteProvenance(_))
        ));
    }
}

#[test]
fn registry_import_provenance_requires_a_source_lockfile() {
    for imported_from in [None, Some(""), Some(" \t ")] {
        let mut lock = Lockfile::new();
        let mut package = Package::new("serde".into(), "1.0.0".into(), "".into(), "".into());
        package.ecosystem = EcosystemTag::Rust;
        package.provenance = Some(crate::Provenance {
            source_kind: crate::SOURCE_KIND_REGISTRY_IMPORT.to_string(),
            tool: None,
            imported_from: imported_from.map(str::to_string),
        });
        lock.add_package(package);

        assert!(matches!(
            verify_ownership_completeness(&lock),
            Err(LockfileError::IncompleteProvenance(_))
        ));
    }
}

#[test]
fn registry_import_with_nonempty_source_lockfile_is_complete() {
    let mut lock = Lockfile::new();
    let mut package = Package::new("serde".into(), "1.0.0".into(), "".into(), "".into());
    package.ecosystem = EcosystemTag::Rust;
    package.provenance = Some(crate::Provenance {
        source_kind: crate::SOURCE_KIND_REGISTRY_IMPORT.to_string(),
        tool: None,
        imported_from: Some("Cargo.lock".to_string()),
    });
    lock.add_package(package);

    assert!(verify_ownership_completeness(&lock).is_ok());
}
