//! Focused installed binary regression tests.
//! Test hồi quy tập trung cho installed binary.

use super::*;

fn fixture() -> tempfile::TempDir {
    let project = tempfile::tempdir().expect("isolated MGC project");
    let package = project.path().join("node_modules/pkg");
    let bin_dir = project.path().join("node_modules/.bin");
    std::fs::create_dir_all(package.join("bin")).expect("create installed package");
    std::fs::create_dir_all(&bin_dir).expect("create npm bin directory");
    std::fs::write(
        package.join("package.json"),
        r#"{"name":"pkg","version":"1.2.3","bin":{"pkg":"bin/pkg.js"}}"#,
    )
    .expect("write installed package manifest");
    let target = package.join("bin/pkg.js");
    std::fs::write(&target, "#!/usr/bin/env node\n").expect("write declared executable");
    #[cfg(unix)]
    std::os::unix::fs::symlink(&target, bin_dir.join("pkg")).expect("create owned bin symlink");
    #[cfg(windows)]
    std::fs::write(bin_dir.join("pkg.cmd"), "@echo off\n").expect("create Windows bin shim");
    project
}

fn workspace(project: &tempfile::TempDir) -> RunnerWorkspace {
    let directory = tempfile::tempdir().expect("lifetime guard");
    RunnerWorkspace {
        _directory: directory,
        project_root: project.path().to_path_buf(),
        canonical_project: project.path().canonicalize().expect("canonical project"),
    }
}

#[test]
fn verifies_owned_package_and_declared_executable() {
    let project = fixture();
    let workspace = workspace(&project);
    let verified =
        verify_installed_binary(&workspace, "pkg", None).expect("verify package and executable");
    assert_eq!(verified.package_version, "1.2.3");
    assert_eq!(verified.bin_name, "pkg");
    assert!(
        verified
            .package_root
            .starts_with(&workspace.canonical_project)
    );
    assert!(
        verified
            .canonical_target
            .starts_with(&verified.package_root)
    );
    assert!(verified.bin_path.exists());
}

#[test]
fn rejects_declared_target_outside_package_root() {
    let project = fixture();
    let package = project.path().join("node_modules/pkg");
    std::fs::write(
        project.path().join("node_modules/escape.js"),
        "outside package",
    )
    .expect("write project-local but package-foreign file");
    std::fs::write(
        package.join("package.json"),
        r#"{"name":"pkg","version":"1.2.3","bin":{"pkg":"../escape.js"}}"#,
    )
    .expect("write escaping package manifest");
    let workspace = workspace(&project);
    assert!(verify_installed_binary(&workspace, "pkg", None).is_err());
}

#[cfg(unix)]
#[test]
fn rejects_bin_symlink_outside_project() {
    let project = fixture();
    let outside = tempfile::NamedTempFile::new().expect("outside executable");
    let shim = project.path().join("node_modules/.bin/pkg");
    std::fs::remove_file(&shim).expect("remove owned shim");
    std::os::unix::fs::symlink(outside.path(), &shim).expect("redirect shim outside");
    let workspace = workspace(&project);
    assert!(verify_installed_binary(&workspace, "pkg", None).is_err());
}

#[cfg(unix)]
#[test]
fn rejects_bin_symlink_to_another_project_file() {
    let project = fixture();
    let unrelated = project.path().join("unrelated");
    std::fs::write(&unrelated, "not the declared executable").expect("write unrelated file");
    let shim = project.path().join("node_modules/.bin/pkg");
    std::fs::remove_file(&shim).expect("remove owned shim");
    std::os::unix::fs::symlink(&unrelated, &shim).expect("redirect shim inside project");
    let workspace = workspace(&project);
    assert!(verify_installed_binary(&workspace, "pkg", None).is_err());
}
