//! Tests for absolute Flutter project URIs and caller-visible aliases.
//! Test URI tuyệt đối và alias project theo đường gọi của Flutter.

#[cfg(unix)]
#[test]
fn flutter_package_root_uri_makes_relative_alias_absolute_without_resolving_it() {
    let current_dir = tempfile::tempdir().unwrap();
    let project = tempfile::tempdir().unwrap();
    let alias = current_dir.path().join("project-alias");
    std::os::unix::fs::symlink(project.path(), &alias).unwrap();

    let relative_uri = super::flutter_project_root_uri(
        std::path::Path::new("project-alias"),
        Some(current_dir.path()),
    )
    .unwrap();
    let absolute_uri = super::flutter_project_root_uri(&alias, None).unwrap();

    assert_eq!(
        relative_uri,
        url::Url::from_directory_path(&alias).unwrap().to_string(),
        "relative roots need an absolute URI while retaining the caller's symlink path"
    );
    assert_eq!(
        absolute_uri, relative_uri,
        "absolute roots must not need the process current directory"
    );
}
