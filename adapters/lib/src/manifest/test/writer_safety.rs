//! Native manifest writer filesystem safety tests.
//! Test an toàn filesystem cho writer manifest native.

use super::support::*;

#[cfg(unix)]
#[test]
fn native_cargo_go_and_python_writers_refuse_symlink_manifests() {
    use crate::manifest::{write_cargo_manifest, write_go_mod_manifest, write_pyproject_manifest};
    use std::os::unix::fs::symlink;

    for (manifest_name, writer) in [
        (
            "Cargo.toml",
            write_cargo_manifest
                as fn(&std::path::Path, &mgc_types::Manifest) -> mgc_types::MgResult<()>,
        ),
        ("go.mod", write_go_mod_manifest),
        ("pyproject.toml", write_pyproject_manifest),
    ] {
        let project = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let external_manifest = outside.path().join("manifest");
        std::fs::write(&external_manifest, "untouched = true\n").unwrap();
        symlink(&external_manifest, project.path().join(manifest_name)).unwrap();

        let error = writer(
            project.path(),
            &mgc_types::Manifest::new("demo", mgc_types::Ecosystem::Lib),
        )
        .unwrap_err();

        assert!(error.to_string().contains("link") || error.to_string().contains("regular"));
        assert_eq!(
            std::fs::read_to_string(external_manifest).unwrap(),
            "untouched = true\n"
        );
    }
}
#[cfg(unix)]
#[test]
fn native_maven_writer_refuses_symlink_manifest_without_touching_target() {
    use crate::manifest::write_pom_manifest;
    use mgc_types::{Ecosystem, Manifest};
    use std::os::unix::fs::symlink;

    let project = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    let target = outside.path().join("important.xml");
    std::fs::write(&target, "outside sentinel").unwrap();
    symlink(&target, project.path().join("pom.xml")).unwrap();

    let result = write_pom_manifest(project.path(), &Manifest::new("demo", Ecosystem::Lib));
    assert!(result.is_err());
    assert_eq!(std::fs::read_to_string(target).unwrap(), "outside sentinel");
    assert!(
        std::fs::symlink_metadata(project.path().join("pom.xml"))
            .unwrap()
            .file_type()
            .is_symlink()
    );
}
#[cfg(unix)]
#[test]
fn dotnet_native_ownership_and_writer_ignore_symlink_csproj() {
    use crate::manifest::write_csproj_manifest;
    use mgc_types::{Ecosystem, Manifest};
    use std::os::unix::fs::symlink;

    let project = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    let target = outside.path().join("important.csproj");
    std::fs::write(&target, "<Project>outside sentinel</Project>").unwrap();
    symlink(&target, project.path().join("Demo.csproj")).unwrap();

    assert_eq!(
        dependency_manifest_format(project.path(), LibLanguage::DotNet),
        None
    );
    let result = write_csproj_manifest(project.path(), &Manifest::new("demo", Ecosystem::Lib));
    assert!(result.is_err());
    assert_eq!(
        std::fs::read_to_string(target).unwrap(),
        "<Project>outside sentinel</Project>"
    );
}
#[cfg(unix)]
#[test]
fn maven_atomic_rewrite_preserves_existing_manifest_mode() {
    use crate::manifest::write_pom_manifest;
    use mgc_types::{Ecosystem, Manifest};
    use std::os::unix::fs::PermissionsExt;

    let project = tempfile::tempdir().unwrap();
    let pom = project.path().join("pom.xml");
    std::fs::write(&pom, "<project/>").unwrap();
    std::fs::set_permissions(&pom, std::fs::Permissions::from_mode(0o600)).unwrap();
    write_pom_manifest(project.path(), &Manifest::new("demo", Ecosystem::Lib)).unwrap();
    assert_eq!(
        std::fs::metadata(pom).unwrap().permissions().mode() & 0o777,
        0o600
    );
}
