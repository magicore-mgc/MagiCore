//! The stable facade reaches both native Cargo operations.
//! Facade ổn định gọi được hai thao tác Cargo native.

use super::{parse_manifest, write_manifest};
use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageName, VersionRange};

#[test]
fn public_facade_round_trips_a_supported_root_dependency() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n[dependencies]\n",
    )
    .unwrap();

    let mut manifest = Manifest::new("demo", Ecosystem::Lib);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("serde").unwrap(),
            VersionRange::parse("1.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    write_manifest(project.path(), &manifest).unwrap();

    let parsed = parse_manifest(project.path(), Ecosystem::Lib).unwrap();
    assert_eq!(parsed.find_dep("serde").unwrap().range.as_str(), "1.0");
}
