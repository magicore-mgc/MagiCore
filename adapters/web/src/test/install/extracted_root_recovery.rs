//! Web extracted-root recovery integration tests.
//! Test tích hợp recovery extracted-root Web.

use super::*;

#[tokio::test]
async fn test_install_rebuilds_shared_extracted_root_when_marker_mismatches() {
    let shared = tempdir_real().unwrap();

    let first = tempdir_real().unwrap();
    std::fs::write(
        first.path().join("package.json"),
        serde_json::json!({
            "name": "demo-a",
            "version": "0.1.0",
            "dependencies": {
                "react": "^18.2.0"
            }
        })
        .to_string(),
    )
    .unwrap();

    let react = PackageId::new(
        PackageName::new("react").unwrap(),
        Version::parse("18.2.0").unwrap(),
    );
    let integrity = seed_cached_tarball_with_files(
        first.path(),
        &react,
        &[
            (
                "package/package.json",
                br#"{"name":"react","version":"18.2.0"}"#.as_slice(),
            ),
            ("package/index.js", b"export default 'react';\n"),
        ],
    );

    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: react.clone(),
            integrity: integrity.clone(),
            tarball_url: String::new(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };

    let adapter = WebAdapter::with_registry_and_shared_cache(
        TEST_REGISTRY_URL.into(),
        shared.path().to_path_buf(),
    );
    adapter
        .install(&graph, first.path(), InstallOptions::default())
        .await
        .unwrap();

    let shared_root = shared_extracted_package_root(shared.path(), &graph.packages[0]);
    std::fs::write(shared_root.join("index.js"), "tampered\n").unwrap();
    write_extracted_package_marker(
        &shared_root,
        &ExtractedPackageMarker {
            schema_version: 0,
            name: "react".into(),
            version: "18.2.0".into(),
            integrity: Some(integrity),
            tarball_sha256: "bad-digest".into(),
            file_count: 0,
            unpacked_size: 0,
            file_tree_sha256: "bad-tree".into(),
        },
    )
    .unwrap();

    let second = tempdir_real().unwrap();
    std::fs::write(
        second.path().join("package.json"),
        serde_json::json!({
            "name": "demo-b",
            "version": "0.1.0",
            "dependencies": {
                "react": "^18.2.0"
            }
        })
        .to_string(),
    )
    .unwrap();
    seed_cached_tarball_with_files(
        second.path(),
        &react,
        &[
            (
                "package/package.json",
                br#"{"name":"react","version":"18.2.0"}"#.as_slice(),
            ),
            ("package/index.js", b"export default 'react';\n"),
        ],
    );

    adapter
        .install(&graph, second.path(), InstallOptions::default())
        .await
        .unwrap();

    assert_eq!(
        std::fs::read_to_string(shared_root.join("index.js")).unwrap(),
        "export default 'react';\n"
    );
    assert_eq!(
        std::fs::read_to_string(second.path().join("node_modules/react/index.js")).unwrap(),
        "export default 'react';\n"
    );
    let marker = read_extracted_package_marker(&shared_root)
        .unwrap()
        .unwrap();
    assert_ne!(marker.tarball_sha256, "bad-digest");
}

#[tokio::test]
async fn test_install_rebuilds_cached_root_when_file_tree_is_incomplete() {
    let shared = tempdir_real().unwrap();
    let project = tempdir_real().unwrap();
    let rollup = PackageId::new(
        PackageName::new("rollup").unwrap(),
        Version::parse("4.62.2").unwrap(),
    );
    let integrity = seed_shared_tarball_with_files(
            shared.path(),
            &rollup,
            &[
                (
                    "package/package.json",
                    br#"{"name":"rollup","version":"4.62.2","exports":{"./parseAst":{"import":"./dist/es/parseAst.js","require":"./dist/parseAst.js"}}}"#.as_slice(),
                ),
                ("package/dist/parseAst.js", b"module.exports = {};\n"),
                (
                    "package/dist/es/parseAst.js",
                    b"export const parseAst = () => null;\n",
                ),
            ],
        );
    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: rollup.clone(),
            integrity: integrity.clone(),
            tarball_url: String::new(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };
    let shared_root = shared_extracted_package_root(shared.path(), &graph.packages[0]);
    std::fs::create_dir_all(shared_root.join("dist")).unwrap();
    std::fs::write(
        shared_root.join("package.json"),
        br#"{"name":"rollup","version":"4.62.2"}"#,
    )
    .unwrap();
    std::fs::write(
        shared_root.join("dist/parseAst.js"),
        b"module.exports = {};\n",
    )
    .unwrap();
    let tarball = PackageCache::for_registry(shared.path().join("cache"), TEST_REGISTRY_URL)
        .unwrap()
        .get_tarball(&rollup)
        .unwrap()
        .unwrap();
    let mut marker =
        expected_extracted_package_marker_from_bytes(&graph.packages[0], &tarball).unwrap();
    marker.schema_version = 0;
    marker.file_count = 0;
    marker.unpacked_size = 0;
    marker.file_tree_sha256.clear();
    write_extracted_package_marker(&shared_root, &marker).unwrap();

    let adapter = WebAdapter::with_registry_and_shared_cache(
        TEST_REGISTRY_URL.into(),
        shared.path().to_path_buf(),
    );
    adapter
        .install(&graph, project.path(), InstallOptions::default())
        .await
        .unwrap();

    assert!(
        project
            .path()
            .join("node_modules/rollup/dist/es/parseAst.js")
            .exists()
    );
}

#[tokio::test]
async fn test_install_rebuilds_legacy_root_when_marker_signature_is_missing() {
    let shared = tempdir_real().unwrap();
    let project = tempdir_real().unwrap();
    let entities = PackageId::new(
        PackageName::new("entities").unwrap(),
        Version::parse("7.0.1").unwrap(),
    );
    let integrity = seed_shared_tarball_with_files(
            shared.path(),
            &entities,
            &[
                (
                    "package/package.json",
                    br#"{"name":"entities","version":"7.0.1","exports":{"./decode":{"require":{"default":"./dist/commonjs/decode.js"}}}}"#.as_slice(),
                ),
                ("package/decode.js", b"module.exports = {};\n"),
                ("package/dist/commonjs/decode.js", b"module.exports = {};\n"),
            ],
        );
    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: entities.clone(),
            integrity: integrity.clone(),
            tarball_url: String::new(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };
    let shared_root = shared_extracted_package_root(shared.path(), &graph.packages[0]);
    std::fs::create_dir_all(&shared_root).unwrap();
    std::fs::write(
            shared_root.join("package.json"),
            br#"{"name":"entities","version":"7.0.1","exports":{"./decode":{"require":{"default":"./dist/commonjs/decode.js"}}}}"#,
        )
        .unwrap();
    std::fs::write(shared_root.join("decode.js"), b"module.exports = {};\n").unwrap();

    let tarball = PackageCache::for_registry(shared.path().join("cache"), TEST_REGISTRY_URL)
        .unwrap()
        .get_tarball(&entities)
        .unwrap()
        .unwrap();
    let mut marker =
        expected_extracted_package_marker_from_bytes(&graph.packages[0], &tarball).unwrap();
    marker.file_count = 0;
    marker.unpacked_size = 0;
    marker.file_tree_sha256.clear();
    write_extracted_package_marker(&shared_root, &marker).unwrap();

    let adapter = WebAdapter::with_registry_and_shared_cache(
        TEST_REGISTRY_URL.into(),
        shared.path().to_path_buf(),
    );
    adapter
        .install(&graph, project.path(), InstallOptions::default())
        .await
        .unwrap();

    assert!(
        project
            .path()
            .join("node_modules/entities/dist/commonjs/decode.js")
            .exists()
    );
}

#[tokio::test]
async fn test_default_cache_validation_rebuilds_v3_root_when_file_tree_is_incomplete() {
    let shared = tempdir_real().unwrap();
    let project = tempdir_real().unwrap();
    let rollup = PackageId::new(
        PackageName::new("rollup").unwrap(),
        Version::parse("4.62.2").unwrap(),
    );
    let integrity = seed_shared_tarball_with_files(
        shared.path(),
        &rollup,
        &[
            (
                "package/package.json",
                br#"{"name":"rollup","version":"4.62.2"}"#.as_slice(),
            ),
            ("package/dist/parseAst.js", b"module.exports = {};\n"),
            (
                "package/dist/es/parseAst.js",
                b"export const parseAst = () => null;\n",
            ),
        ],
    );
    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: rollup.clone(),
            integrity: integrity.clone(),
            tarball_url: String::new(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };
    let shared_root = shared_extracted_package_root(shared.path(), &graph.packages[0]);
    std::fs::create_dir_all(shared_root.join("dist")).unwrap();
    std::fs::write(
        shared_root.join("package.json"),
        br#"{"name":"rollup","version":"4.62.2"}"#,
    )
    .unwrap();
    std::fs::write(
        shared_root.join("dist/parseAst.js"),
        b"module.exports = {};\n",
    )
    .unwrap();
    let tarball = PackageCache::for_registry(shared.path().join("cache"), TEST_REGISTRY_URL)
        .unwrap()
        .get_tarball(&rollup)
        .unwrap()
        .unwrap();
    let marker =
        expected_extracted_package_marker_from_bytes(&graph.packages[0], &tarball).unwrap();
    write_extracted_package_marker(&shared_root, &marker).unwrap();

    let adapter = WebAdapter::with_registry_and_shared_cache(
        TEST_REGISTRY_URL.into(),
        shared.path().to_path_buf(),
    );
    adapter
        .install(&graph, project.path(), InstallOptions::default())
        .await
        .unwrap();

    assert!(
        project
            .path()
            .join("node_modules/rollup/dist/es/parseAst.js")
            .exists()
    );
}
