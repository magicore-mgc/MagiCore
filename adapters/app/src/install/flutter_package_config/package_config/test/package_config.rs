//! Flutter package-config generation, root identity, and output-path tests.
//! Test tạo package config Flutter, danh tính root và đường dẫn output.

use super::*;

#[test]
fn flutter_install_writes_package_config_for_verified_materialized_graph() {
    let project = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    std::fs::create_dir(project.path().join("lib")).unwrap();
    std::fs::write(
        project.path().join("pubspec.yaml"),
        "name: sample\nversion: 1.0.0\ndependencies:\n  http: ^1.2.0\ndev_dependencies:\n  meta: ^1.0.0\n",
    )
    .unwrap();
    let package_root = cache.path().join("hosted/pub.dev/http-1.2.0");
    std::fs::create_dir_all(package_root.join("lib")).unwrap();
    std::fs::write(
        package_root.join("pubspec.yaml"),
        "name: http\nenvironment:\n  sdk: '>=3.2.0 <4.0.0'\n",
    )
    .unwrap();
    let meta_root = cache.path().join("hosted/pub.dev/meta-1.0.0");
    std::fs::create_dir_all(meta_root.join("lib")).unwrap();
    std::fs::write(meta_root.join("pubspec.yaml"), "name: meta\n").unwrap();
    let graph = ResolvedGraph {
        packages: vec![
            ResolvedPackage {
                id: PackageId::new(
                    PackageName::new("http").unwrap(),
                    Version::parse("1.2.0").unwrap(),
                ),
                integrity: "sha256-fixture".to_string(),
                tarball_url: "https://pub.dev/http.tar.gz".to_string(),
                deps: vec![PackageId::new(
                    PackageName::new("meta").unwrap(),
                    Version::parse("1.0.0").unwrap(),
                )],
                peer_deps: vec![],
                direct: true,
                dev: false,
            },
            ResolvedPackage {
                id: PackageId::new(
                    PackageName::new("meta").unwrap(),
                    Version::parse("1.0.0").unwrap(),
                ),
                integrity: "sha256-meta-fixture".to_string(),
                tarball_url: "https://pub.dev/meta.tar.gz".to_string(),
                deps: vec![],
                peer_deps: vec![],
                direct: false,
                dev: true,
            },
        ],
    };

    write_flutter_package_config(&graph, project.path(), cache.path()).unwrap();

    let config: serde_json::Value = serde_json::from_slice(
        &std::fs::read(project.path().join(".dart_tool/package_config.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(config["configVersion"], 2);
    assert_eq!(config["generator"], "MagiCore");
    let packages = config["packages"].as_array().unwrap();
    assert_eq!(
        packages
            .iter()
            .filter_map(|entry| entry["name"].as_str())
            .collect::<Vec<_>>(),
        ["http", "meta", "sample"]
    );
    let package = |name: &str| packages.iter().find(|entry| entry["name"] == name).unwrap();
    assert_eq!(package("http")["packageUri"], "lib/");
    assert_eq!(package("http")["languageVersion"], "3.2");
    assert!(
        package("http")["rootUri"]
            .as_str()
            .unwrap()
            .starts_with("file://")
    );
    assert_eq!(package("sample")["packageUri"], "lib/");
    assert_eq!(
        package("sample")["rootUri"],
        url::Url::from_directory_path(project.path())
            .unwrap()
            .as_str()
    );

    let package_graph: serde_json::Value = serde_json::from_slice(
        &std::fs::read(project.path().join(".dart_tool/package_graph.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(package_graph["configVersion"], 1);
    assert_eq!(package_graph["roots"], serde_json::json!(["sample"]));
    assert_eq!(
        package_graph["packages"],
        serde_json::json!([
            {
                "name": "sample",
                "version": "1.0.0",
                "dependencies": ["http"],
                "devDependencies": ["meta"]
            },
            {"name": "http", "version": "1.2.0", "dependencies": ["meta"]},
            {"name": "meta", "version": "1.0.0", "dependencies": []}
        ])
    );
}

#[cfg(unix)]
#[test]
fn flutter_package_config_preserves_project_root_alias() {
    let project = tempfile::tempdir().unwrap();
    let parent = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    std::fs::write(project.path().join("pubspec.yaml"), "name: sample\n").unwrap();
    let alias = parent.path().join("project-alias");
    std::os::unix::fs::symlink(project.path(), &alias).unwrap();

    write_flutter_package_config(&ResolvedGraph::empty(), &alias, cache.path()).unwrap();

    let config: serde_json::Value = serde_json::from_slice(
        &std::fs::read(alias.join(".dart_tool/package_config.json")).unwrap(),
    )
    .unwrap();
    let package = config["packages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["name"] == "sample")
        .unwrap();
    assert_eq!(
        package["rootUri"],
        url::Url::from_directory_path(&alias).unwrap().as_str(),
        "Flutter compares the root package URI with its caller-visible project URI"
    );
}

#[cfg(unix)]
#[test]
fn flutter_package_config_refuses_symlinked_dart_tool_directory() {
    use std::os::unix::fs::symlink;

    let project = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    std::fs::write(project.path().join("pubspec.yaml"), "name: sample\n").unwrap();
    symlink(outside.path(), project.path().join(".dart_tool")).unwrap();

    let error =
        write_flutter_package_config(&ResolvedGraph::empty(), project.path(), outside.path())
            .unwrap_err();
    assert!(error.to_string().contains("linked"));
    assert_eq!(std::fs::read_dir(outside.path()).unwrap().count(), 0);
}
