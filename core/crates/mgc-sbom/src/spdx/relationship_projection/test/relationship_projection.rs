#![allow(clippy::unwrap_used)]
//! Relationship projection cases — Trường hợp chiếu relationship

use std::collections::BTreeMap;

use crate::cyclonedx::{Bom, Component, ComponentType, Dependency};
use crate::spdx::error::SpdxError;
use crate::spdx::package_identity::build_package_ids;
use crate::spdx::package_projection::project_packages;
use crate::spdx::relationship_projection::project_relationships;

fn component(bom_ref: &str) -> Component {
    Component {
        component_type: ComponentType::Library,
        bom_ref: bom_ref.into(),
        name: bom_ref.into(),
        version: "1".into(),
        purl: None,
        hashes: None,
        licenses: None,
    }
}

#[test]
fn creates_document_and_dependency_edges_in_deterministic_order() {
    let bom = Bom {
        components: vec![component("pkg:a"), component("pkg:b")],
        dependencies: Some(vec![Dependency {
            dependency_ref: "pkg:a".into(),
            depends_on: Some(vec!["pkg:b".into()]),
        }]),
        ..Bom::default()
    };
    let ids = build_package_ids(&bom).unwrap();
    let packages = project_packages(&bom, &ids).unwrap();
    let (describes, relationships) = project_relationships(&bom, &ids, &packages).unwrap();

    assert_eq!(describes.len(), 2);
    assert_eq!(relationships.len(), 3);
    assert!(relationships.windows(2).all(|pair| pair[0] <= pair[1]));
}

#[test]
fn rejects_missing_dependency_sources_and_targets() {
    let mut bom = Bom {
        dependencies: Some(vec![Dependency {
            dependency_ref: "pkg:missing".into(),
            depends_on: None,
        }]),
        ..Bom::default()
    };
    assert!(matches!(
        project_relationships(&bom, &BTreeMap::new(), &[]),
        Err(SpdxError::DependencySourceMissing(reference)) if reference == "pkg:missing"
    ));

    bom.components.push(component("pkg:a"));
    bom.dependencies = Some(vec![Dependency {
        dependency_ref: "pkg:a".into(),
        depends_on: Some(vec!["pkg:missing".into()]),
    }]);
    let ids = build_package_ids(&bom).unwrap();
    let packages = project_packages(&bom, &ids).unwrap();
    assert!(matches!(
        project_relationships(&bom, &ids, &packages),
        Err(SpdxError::DependencyTargetMissing(reference)) if reference == "pkg:missing"
    ));
}
