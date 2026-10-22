#![allow(clippy::unwrap_used)]
//! Package projection cases — Trường hợp chiếu package

use super::super::project_packages;
use crate::cyclonedx::{Bom, Component, ComponentType};
use crate::spdx::package_identity::build_package_ids;

fn component() -> Component {
    Component {
        component_type: ComponentType::Library,
        bom_ref: "pkg:cargo/demo@1.0.0".into(),
        name: "demo".into(),
        version: "1.0.0".into(),
        purl: Some("pkg:cargo/demo@1.0.0".into()),
        hashes: None,
        licenses: None,
    }
}

#[test]
fn projects_package_metadata_and_package_manager_reference() {
    let mut bom = Bom::default();
    bom.components.push(component());
    let ids = build_package_ids(&bom).unwrap();
    let packages = project_packages(&bom, &ids).unwrap();

    assert_eq!(packages[0].name, "demo");
    assert_eq!(packages[0].version_info.as_deref(), Some("1.0.0"));
    assert_eq!(packages[0].download_location, "NOASSERTION");
    assert_eq!(
        packages[0].external_refs[0].reference_locator,
        "pkg:cargo/demo@1.0.0"
    );
}

#[test]
fn omits_whitespace_purls_and_empty_versions() {
    let mut item = component();
    item.purl = Some("pkg:cargo/demo with-space".into());
    item.version.clear();
    let mut bom = Bom::default();
    bom.components.push(item);
    let ids = build_package_ids(&bom).unwrap();
    let packages = project_packages(&bom, &ids).unwrap();

    assert!(packages[0].version_info.is_none());
    assert!(packages[0].external_refs.is_empty());
}
