#![allow(clippy::unwrap_used)]
//! Package identity cases — Trường hợp identity package

use super::super::super::*;
use crate::cyclonedx::{Bom, Component};
use crate::spdx::error::SpdxError;

fn component(bom_ref: &str, name: &str) -> Component {
    Component {
        component_type: crate::cyclonedx::ComponentType::Library,
        bom_ref: bom_ref.into(),
        name: name.into(),
        version: "1".into(),
        purl: None,
        hashes: None,
        licenses: None,
    }
}

#[test]
fn assigns_deterministic_ids_to_distinct_bom_references() {
    let bom = Bom {
        components: vec![component("pkg:a", "a"), component("pkg:b", "b")],
        ..Bom::default()
    };
    let first = build_package_ids(&bom).unwrap();
    let second = build_package_ids(&bom).unwrap();

    assert_eq!(first, second);
    assert_ne!(first["pkg:a"], first["pkg:b"]);
}

#[test]
fn rejects_empty_names_and_duplicate_bom_references() {
    let mut bom = Bom::default();
    bom.components.push(component("pkg:a", " "));
    assert!(matches!(
        build_package_ids(&bom),
        Err(SpdxError::EmptyPackageName)
    ));

    bom.components = vec![component("pkg:a", "a"), component("pkg:a", "b")];
    assert!(matches!(
        build_package_ids(&bom),
        Err(SpdxError::DuplicatePackageIdentity(reference)) if reference == "pkg:a"
    ));
}
