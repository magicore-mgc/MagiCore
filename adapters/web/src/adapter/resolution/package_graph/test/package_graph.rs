//! Exact direct-root graph assembly tests / test assembly root chính xác.

use super::super::direct_root_ids;
use mgc_resolver::solver::{Resolution, RootResolution};
use mgc_types::{PackageId, PackageName};

fn resolution(package: &str) -> Resolution {
    let package_id = PackageId::parse(package).expect("parse package id");
    Resolution {
        version: package_id.version().clone(),
        package_id,
        integrity: String::new(),
        deps: Vec::new(),
        dep_specs: Vec::new(),
        peer_deps: Vec::new(),
    }
}

fn root_resolution(package: &str) -> RootResolution {
    let package_id = PackageId::parse(package).expect("parse root package id");
    RootResolution {
        request_index: 0,
        request_name: PackageName::new(package_id.name_str()).expect("parse request name"),
        request_spec: "^1.0.0".to_owned(),
        package_id,
    }
}

#[test]
fn direct_flag_uses_only_the_indexed_root_package_identity() {
    let root = root_resolution("widget@1.2.0");
    let transitive = resolution("widget@1.9.0");
    let root_id = root.package_id.clone();
    let transitive_id = transitive.package_id.clone();

    let direct_ids = direct_root_ids(&[root_id_resolution(&root), transitive], &[root])
        .expect("root selection exists in the resolution set");

    assert!(direct_ids.contains(&root_id));
    assert!(!direct_ids.contains(&transitive_id));
}

#[test]
fn missing_resolver_root_refuses_to_guess_from_a_same_name_package() {
    let root = root_resolution("widget@1.2.0");
    let error = direct_root_ids(&[resolution("widget@1.9.0")], &[root])
        .expect_err("missing exact root must not promote another same-name package");

    assert!(error.to_string().contains("widget@1.2.0"));
}

fn root_id_resolution(root: &RootResolution) -> Resolution {
    resolution(&root.package_id.to_string())
}
