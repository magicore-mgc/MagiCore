// cache_tests — split from unit_tests.rs by behavior/subject (P07).
// cache_tests — tách từ unit_tests.rs theo behavior/chủ đề (P07).
use super::*;







#[test]
fn test_manifest_resolution_cache_key_ignores_dep_order_and_app_name() {
    let registry_url = "https://registry.npmjs.org";
    let mut left = Manifest::new("demo-a", mgc_types::ecosystem::Ecosystem::Web);
    left.add_dep(
        DependencySpec::new(
            PackageName::new("react").unwrap(),
            VersionRange::parse("^18.2.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    left.add_dep(
        DependencySpec::new(
            PackageName::new("zod").unwrap(),
            VersionRange::parse("^3.22.4").unwrap(),
        ),
        true,
        false,
        false,
    );

    let mut right = Manifest::new("demo-b", mgc_types::ecosystem::Ecosystem::Web);
    right.add_dep(
        DependencySpec::new(
            PackageName::new("zod").unwrap(),
            VersionRange::parse("^3.22.4").unwrap(),
        ),
        true,
        false,
        false,
    );
    right.add_dep(
        DependencySpec::new(
            PackageName::new("react").unwrap(),
            VersionRange::parse("^18.2.0").unwrap(),
        ),
        false,
        false,
        false,
    );

    assert_eq!(
        manifest_resolution_cache_key(&left, registry_url),
        manifest_resolution_cache_key(&right, registry_url)
    );
}













#[test]
fn streamed_tarball_staging_is_unique_and_publish_is_first_writer_wins() {
    let dir = tempdir_real().unwrap();
    let final_path = dir.path().join("pkg.tgz");
    let payload = b"complete streamed package";
    let integrity = compute_tarball_integrity(payload);
    let barrier = Arc::new(std::sync::Barrier::new(12));
    let staging_paths = Arc::new(std::sync::Mutex::new(Vec::new()));

    let writers = (0..12)
        .map(|_| {
            let final_path = final_path.clone();
            let barrier = Arc::clone(&barrier);
            let staging_paths = Arc::clone(&staging_paths);
            let integrity = integrity.clone();
            std::thread::spawn(move || {
                let staging = unique_tarball_staging_path(&final_path);
                std::fs::write(&staging, payload).unwrap();
                staging_paths.lock().unwrap().push(staging.clone());
                barrier.wait();
                publish_downloaded_tarball(&staging, &final_path, &integrity)
            })
        })
        .collect::<Vec<_>>();

    for writer in writers {
        writer.join().unwrap().unwrap();
    }

    let paths = staging_paths.lock().unwrap();
    assert_eq!(
        paths.iter().collect::<std::collections::HashSet<_>>().len(),
        12
    );
    assert!(paths.iter().all(|path| !path.exists()));
    assert_eq!(std::fs::read(final_path).unwrap(), payload);
}





#[test]
fn test_extracted_content_signature_detects_content_and_mode_mutation() {
    let package = ResolvedPackage {
        id: PackageId::new(
            PackageName::new("content-check").unwrap(),
            Version::parse("1.0.0").unwrap(),
        ),
        integrity: String::new(),
        tarball_url: String::new(),
        deps: vec![],
        peer_deps: vec![],
        direct: true,
        dev: false,
    };
    let archive = build_tarball_bytes(&[
        (
            "package/package.json",
            br#"{"name":"content-check","version":"1.0.0"}"#,
        ),
        ("package/index.js", b"const value = 'good';\n"),
    ]);
    let expected = expected_extracted_package_marker_from_bytes(&package, &archive).unwrap();
    assert_eq!(expected.schema_version, 3);
    let root = tempdir_real().unwrap();
    std::fs::write(
        root.path().join("package.json"),
        br#"{"name":"content-check","version":"1.0.0"}"#,
    )
    .unwrap();
    std::fs::write(root.path().join("index.js"), b"const value = 'good';\n").unwrap();
    write_extracted_package_marker(root.path(), &expected).unwrap();

    assert!(extracted_content_matches(root.path(), &expected).unwrap());
    assert!(materialized_package_matches(root.path(), &package.id, Some(&expected)).unwrap());
    std::fs::write(root.path().join("index.js"), b"const value = 'evil';\n").unwrap();

    assert!(!extracted_content_matches(root.path(), &expected).unwrap());
    assert!(!materialized_package_matches(root.path(), &package.id, Some(&expected)).unwrap());
    std::fs::write(root.path().join("index.js"), b"const value = 'good';\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(
            root.path().join("index.js"),
            std::fs::Permissions::from_mode(0o755),
        )
        .unwrap();
        assert!(!extracted_content_matches(root.path(), &expected).unwrap());
    }
}

#[cfg(unix)]
#[test]
fn test_extracted_content_signature_rejects_symlink_files() {
    let package = ResolvedPackage {
        id: PackageId::new(
            PackageName::new("symlink-check").unwrap(),
            Version::parse("1.0.0").unwrap(),
        ),
        integrity: String::new(),
        tarball_url: String::new(),
        deps: vec![],
        peer_deps: vec![],
        direct: true,
        dev: false,
    };
    let archive = build_tarball_bytes(&[
        (
            "package/package.json",
            br#"{"name":"symlink-check","version":"1.0.0"}"#,
        ),
        ("package/index.js", b"module.exports = true;\n"),
    ]);
    let expected = expected_extracted_package_marker_from_bytes(&package, &archive).unwrap();
    let root = tempdir_real().unwrap();
    std::fs::write(
        root.path().join("package.json"),
        br#"{"name":"symlink-check","version":"1.0.0"}"#,
    )
    .unwrap();
    std::os::unix::fs::symlink("/etc/hosts", root.path().join("index.js")).unwrap();

    assert!(!extracted_content_matches(root.path(), &expected).unwrap());
}
