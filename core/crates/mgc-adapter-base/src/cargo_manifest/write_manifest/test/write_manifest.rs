//! Writer leaves unrelated Cargo sections intact.
//! Writer giữ nguyên các section Cargo không thuộc phạm vi cập nhật.

use super::write_manifest;
use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageName, VersionRange};

#[test]
fn writes_root_dependencies_without_erasing_target_or_feature_sections() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        concat!(
            "[package]\n",
            "name = \"demo\"\n",
            "version = \"0.1.0\"\n",
            "[dependencies]\nold = \"0.1\"\n",
            "[target.'cfg(unix)'.dependencies]\n",
            "libc = \"0.2\"\n[features]\ndefault = []\n",
        ),
    )
    .unwrap();
    let mut manifest = Manifest::new("demo", Ecosystem::Lib);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("serde").unwrap(),
            VersionRange::parse("1.5").unwrap(),
        ),
        false,
        false,
        false,
    );

    write_manifest(project.path(), &manifest).unwrap();

    let value: toml::Value =
        toml::from_str(&std::fs::read_to_string(project.path().join("Cargo.toml")).unwrap())
            .unwrap();
    assert!(value["dependencies"].get("serde").is_some());
    assert!(value["dependencies"].get("old").is_none());
    assert!(value["target"]["cfg(unix)"]["dependencies"]["libc"].is_str());
    assert!(value["features"]["default"].is_array());
}
