//! App lock replay tests — kiểm thử replay lock của App.
//! Owns explicit non-SDK requirements in lock replay. — Sở hữu dependency ngoài SDK khi replay lock.

use mgc_app_adapter::{AppAdapter, AppLanguage};
use mgc_types::PackageName;
use mgc_types::adapter::PackageAdapter;

pub(crate) fn lock_replay_manifest_preserves_explicit_non_sdk_requirements() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("pubspec.yaml"),
        "name: app\ndependencies:\n  http: ^1.0.0\n",
    )
    .unwrap();
    let mut manifest = mgc_types::Manifest::new("app", mgc_types::Ecosystem::App);
    manifest.add_dep(
        mgc_types::DependencySpec::new(
            PackageName::new("http").unwrap(),
            mgc_types::VersionRange::parse("^1.0.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    for language in [
        AppLanguage::Flutter,
        AppLanguage::Swift,
        AppLanguage::Kotlin,
    ] {
        let adapter = AppAdapter::new(language);
        let replay = adapter
            .lock_replay_manifest(project.path(), &manifest)
            .unwrap();
        assert_eq!(replay.name, manifest.name);
        assert_eq!(replay.all_dependencies().count(), 1);
        assert_eq!(replay.find_dep("http").unwrap().range.to_string(), "^1.0.0");
        assert_eq!(manifest.all_dependencies().count(), 1);
    }
}
