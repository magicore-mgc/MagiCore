#![allow(clippy::unwrap_used)]
//! Identity owner tests — Kiểm thử owner identity

use super::super::*;

#[test]
fn legacy_reference_does_not_expose_the_resolved_url() {
    let package = mgc_lockfile::schema::Package {
        name: "private-lib".to_string(),
        version: "1.0.0".to_string(),
        resolved: "https://user:secret@registry.invalid/pkg.tgz".to_string(),
        ..Default::default()
    };

    let reference = legacy_bom_ref(&package);

    assert!(!reference.contains("registry.invalid"));
    assert!(!reference.contains("secret"));
    assert_eq!(reference, legacy_bom_ref(&package));
}

#[test]
fn bare_legacy_dependency_name_must_be_unique() {
    let refs_by_id = HashMap::new();
    let refs_by_name = HashMap::from([(
        "serde".to_string(),
        vec!["ref-one".to_string(), "ref-two".to_string()],
    )]);

    let result = resolve_legacy_dependency_ref("serde", &refs_by_id, &refs_by_name);

    assert!(matches!(result, Err(GeneratorError::InvalidLockfile(_))));
}
