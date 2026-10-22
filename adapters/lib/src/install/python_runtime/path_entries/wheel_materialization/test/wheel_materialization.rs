//! Authenticated wheel and site-materialization tests.
//! Test xác minh wheel và site đã materialize.

use super::support::*;

#[test]
fn python_runtime_verifies_cached_wheel_and_site_against_lock_before_exposing_path() {
    let project = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    mgc_config::project::ProjectConfig::write_core_marker_at(project.path(), "ai").unwrap();
    std::fs::write(
        project.path().join("pyproject.toml"),
        "[project]\nname = \"runtime-integrity-probe\"\nversion = \"0.1.0\"\ndependencies = [\"demo-pkg==1.2.3\"]\n",
    )
    .unwrap();

    let wheel_name = "demo_pkg-1.2.3-py3-none-any.whl";
    let wheel_url = format!("https://files.pythonhosted.org/packages/{wheel_name}");
    let module_path = "demo_pkg/__init__.py";
    let module_bytes = b"VALUE = 'trusted'\n";
    let wheel = python_wheel("demo_pkg", "1.2.3", module_path, module_bytes);
    let digest = mgc_resolver::protocols::sha256_hex(&wheel);
    let wheels = store.path().join("wheels");
    let artifact_path = wheels.join(
        mgc_resolver::protocols::PypiProtocol::artifact_cache_relpath(&wheel_url, &digest).unwrap(),
    );
    let site_dirname = mgc_resolver::protocols::PypiProtocol::importable_site_dirname_for_digest(
        "demo-pkg", "1.2.3", &digest,
    )
    .unwrap();
    let site_path = wheels.join("site").join(site_dirname);
    std::fs::create_dir_all(site_path.join("demo_pkg")).unwrap();
    std::fs::create_dir_all(site_path.join("demo_pkg-1.2.3.dist-info")).unwrap();
    std::fs::create_dir_all(artifact_path.parent().unwrap()).unwrap();
    std::fs::write(&artifact_path, &wheel).unwrap();
    std::fs::write(site_path.join(module_path), module_bytes).unwrap();
    let mut lock = Lockfile::new();
    let mut pinned = Package::new(
        "demo-pkg".into(),
        "1.2.3".into(),
        wheel_url,
        format!("sha256-{digest}"),
    );
    pinned.ecosystem = EcosystemTag::Python;
    pinned.owner_core = Some("ai".into());
    pinned.registry = Some("https://pypi.org/simple".to_string());
    lock.add_package(pinned);
    std::fs::write(
        project.path().join("mgc.lock"),
        writer::serialize_lockfile(&lock).unwrap(),
    )
    .unwrap();
    std::fs::write(
        site_path.join("demo_pkg-1.2.3.dist-info/METADATA"),
        "Name: demo_pkg\nVersion: 1.2.3\n",
    )
    .unwrap();
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(&wheel)).unwrap();
    let record = archive.by_name("demo_pkg-1.2.3.dist-info/RECORD").unwrap();
    let mut record_bytes = Vec::new();
    record
        .take(16 * 1024)
        .read_to_end(&mut record_bytes)
        .unwrap();
    std::fs::write(
        site_path.join("demo_pkg-1.2.3.dist-info/RECORD"),
        record_bytes,
    )
    .unwrap();

    let paths = native_python_path_entries_from_store(project.path(), store.path()).unwrap();
    assert_eq!(paths.len(), 1);

    // The same installed bytes must verify after lossless V4 migration.
    // Lock V4 đổi encoding không được làm hỏng wheel cache hoặc runtime Python.
    mgc_lockfile::update_owner_root_pins(
        &mut lock,
        "ai",
        EcosystemTag::Python,
        ["demo-pkg@1.2.3".to_string()],
    );
    let (_, warnings) = mgc_lockfile::migrate::migrate_v3_to_v4(lock.clone()).unwrap();
    assert!(
        warnings.is_empty(),
        "valid native digest must survive V4 migration"
    );
    let bytes = mgc_lockfile::migrate::finalize_migrated_v4(
        lock,
        "native-runtime-test",
        "2026-10-11T00:00:00Z",
    )
    .unwrap();
    std::fs::write(project.path().join("mgc.lock"), bytes).unwrap();
    let config_path = project.path().join("mgc.toml");
    let mut config = std::fs::read_to_string(&config_path)
        .unwrap_or_else(|_| "name = \"runtime-integrity-probe\"\necosystem = \"ai\"\n".to_string());
    config.push_str("\n[lock]\nschema = \"v4\"\n");
    std::fs::write(&config_path, config).unwrap();
    let v4_paths = native_python_path_entries_from_store(project.path(), store.path()).unwrap();
    assert_eq!(
        v4_paths, paths,
        "V4 must use the same authenticated wheel materialization"
    );

    // Existing basename-keyed cache entries remain usable only through the
    // same lock digest and authenticated wheel RECORD checks.
    // (Cache cũ chỉ được đọc khi qua cùng kiểm tra digest và RECORD.)
    let legacy_site = wheels.join("site/demo-pkg-1.2.3");
    std::fs::rename(&artifact_path, wheels.join(wheel_name)).unwrap();
    std::fs::rename(&site_path, &legacy_site).unwrap();
    let legacy_paths = native_python_path_entries_from_store(project.path(), store.path()).unwrap();
    assert_eq!(legacy_paths, vec![legacy_site.clone()]);

    std::fs::write(
        legacy_paths[0].join(module_path),
        b"VALUE = 'tampered after install'\n",
    )
    .unwrap();
    let error = native_python_path_entries_from_store(project.path(), store.path()).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("differs from authenticated wheel member")
    );
}
