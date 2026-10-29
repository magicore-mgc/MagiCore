//! Web shared-store link integration tests.
//! Test tích hợp shared-store link Web.

use super::*;

#[cfg(unix)]
#[tokio::test]
async fn test_install_materialization_uses_store_links_from_cached_extract_root() {
    let dir = tempdir_real().unwrap();
    let shared = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        serde_json::json!({
            "name": "demo",
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
        dir.path(),
        &react,
        &[
            (
                "package/package.json",
                br#"{"name":"react","version":"18.2.0"}"#.as_slice(),
            ),
            ("package/index.js", b"export default 'react';"),
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
        .install(&graph, dir.path(), InstallOptions::default())
        .await
        .unwrap();

    let cache_key = extracted_package_cache_key(&graph.packages[0]);
    let cached_root = shared
        .path()
        .join("packages")
        .join("react")
        .join(cache_key)
        .join("package");
    let installed_file = dir
        .path()
        .join("node_modules")
        .join("react")
        .join("index.js");
    let vstore_link = dir
        .path()
        .join("node_modules")
        .join(".magicore")
        .join(format!("react@{}", react.version()))
        .join("node_modules")
        .join("react");

    let link_meta = std::fs::symlink_metadata(&vstore_link)
        .unwrap_or_else(|_| panic!("vstore link not found at: {}", vstore_link.display()));
    assert!(link_meta.file_type().is_dir());
    let refs = read_shared_cache_pinned_package_roots(shared.path());
    assert!(
        refs.contains(&canonical_or_original(&cached_root)),
        "install should pin store-linked package root in shared cache refs"
    );
    assert_eq!(
        std::fs::read_to_string(&installed_file).unwrap(),
        "export default 'react';"
    );
}

#[cfg(unix)]
#[tokio::test]
async fn test_install_repairs_broken_store_links_when_shared_packages_are_deleted() {
    let dir = tempdir_real().unwrap();
    let shared = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        serde_json::json!({
            "name": "demo",
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
        dir.path(),
        &react,
        &[
            (
                "package/package.json",
                br#"{"name":"react","version":"18.2.0"}"#.as_slice(),
            ),
            ("package/index.js", b"export default 'react';"),
        ],
    );

    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: react.clone(),
            integrity,
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
        .install(&graph, dir.path(), InstallOptions::default())
        .await
        .unwrap();

    let installed_file = dir.path().join("node_modules/react/index.js");
    assert!(installed_file.exists());
    std::fs::remove_dir_all(shared.path().join("packages")).unwrap();
    assert!(
        installed_file.exists(),
        "hard-linked install should survive shared cache deletion"
    );

    adapter
        .install(&graph, dir.path(), InstallOptions::default())
        .await
        .unwrap();

    assert_eq!(
        std::fs::read_to_string(&installed_file).unwrap(),
        "export default 'react';"
    );
}
