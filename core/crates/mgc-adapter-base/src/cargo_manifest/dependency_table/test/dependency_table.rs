//! Root table mutation changes only the requested dependency version.
//! Mutation table root chỉ đổi version dependency được yêu cầu.

use super::sync_dependency_table;
use mgc_types::package::{DependencySpec, PackageName, VersionRange};
use mgc_types::{Ecosystem, MgError};

#[test]
fn updates_version_while_preserving_features_and_default_feature_policy() {
    let mut root: toml::Table = toml::from_str(concat!(
        "[dependencies]\n",
        "serde = { version = \"1.0\", features = [\"derive\"], ",
        "default-features = false }\n",
        "keep = \"0.2\"\n",
    ))
    .unwrap();
    let desired = [DependencySpec::new(
        PackageName::new("serde").unwrap(),
        VersionRange::parse("1.5").unwrap(),
    )];

    sync_dependency_table(&mut root, "dependencies", &desired, Ecosystem::Lib).unwrap();

    let serde = root["dependencies"]["serde"].as_table().unwrap();
    assert_eq!(serde["version"].as_str(), Some("1.5"));
    assert_eq!(serde["features"][0].as_str(), Some("derive"));
    assert_eq!(serde["default-features"].as_bool(), Some(false));
    assert!(
        !root["dependencies"]
            .as_table()
            .unwrap()
            .contains_key("keep")
    );
}

#[test]
fn preserves_magicore_metadata_when_removing_unrequested_dependencies() {
    let mut root: toml::Table =
        toml::from_str("[dependencies]\nmagicore = { pinned = true }\nold = \"0.1\"\n").unwrap();

    sync_dependency_table(&mut root, "dependencies", &[], Ecosystem::Lib).unwrap();

    let table = root["dependencies"].as_table().unwrap();
    assert!(table.contains_key("magicore"));
    assert!(!table.contains_key("old"));
}

#[test]
fn unsupported_dependency_reports_the_calling_ecosystem() {
    let mut root: toml::Table =
        toml::from_str("[dependencies]\nserde = { version = \"1.0\", path = \"../serde\" }\n")
            .unwrap();
    let desired = [DependencySpec::new(
        PackageName::new("serde").unwrap(),
        VersionRange::parse("1.5").unwrap(),
    )];

    let error =
        sync_dependency_table(&mut root, "dependencies", &desired, Ecosystem::Game).unwrap_err();

    assert!(matches!(
        &error,
        MgError::Unsupported {
            core: "game",
            capability: "native Cargo dependency mutation",
            ..
        }
    ));
}
