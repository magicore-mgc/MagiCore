//! Age-gate integration contract for delta installs.
//! Contract tích hợp age-gate cho delta install.

use super::super::{AddedPackage, try_install_added_packages_from_lock};
use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageName, VersionRange};

#[tokio::test]
async fn age_gate_disables_delta_install_from_a_previous_lock_graph() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("mgc.toml"),
        "name = \"demo\"\nversion = \"0.1.0\"\necosystem = \"web\"\n[security]\nweb = 24\n",
    )
    .unwrap();
    mgc_config::project::ProjectConfig::write_core_marker_at(project.path(), "web").unwrap();

    let mut previous_manifest = Manifest::new("demo", Ecosystem::Web);
    previous_manifest.add_dep(
        DependencySpec::new(
            PackageName::new("react").unwrap(),
            VersionRange::parse("^18.2.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    let mut lock = mgc_lockfile::Lockfile::new();
    let mut package = mgc_lockfile::Package::new(
        "react".into(),
        "18.2.0".into(),
        "https://registry.example/react.tgz".into(),
        "sha512-react".into(),
    );
    package.owner_core = Some("web".into());
    package.ecosystem = mgc_lockfile::EcosystemTag::Web;
    package.registry = Some("https://registry.example".into());
    lock.add_package(package);
    lock.root_dependencies_by_owner.insert(
        "web".into(),
        vec![mgc_lockfile::format_root_pin(
            mgc_lockfile::EcosystemTag::Web,
            "react@18.2.0",
        )],
    );
    std::fs::write(
        project.path().join("mgc.lock"),
        mgc_lockfile::writer::serialize_lockfile(&lock).unwrap(),
    )
    .unwrap();

    let adapter =
        mgc_web_adapter::WebAdapter::with_registry("https://registry.example".to_string()).unwrap();
    let added = AddedPackage {
        id: mgc_types::PackageId::new(
            PackageName::new("zod").unwrap(),
            mgc_types::Version::parse("3.22.4").unwrap(),
        ),
        dev: false,
        optional: false,
        peer: false,
    };

    let result = try_install_added_packages_from_lock(
        &adapter,
        project.path(),
        Some(&previous_manifest),
        &[added],
    )
    .await;
    assert!(
        matches!(result, Ok(false)),
        "age-gated add must fall back to a full graph resolve, got {result:?}"
    );
}
