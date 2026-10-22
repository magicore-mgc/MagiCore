// lockfile_v4_schema_tests — schema selection and v4/v3 writing.
// lockfile_v4_schema_tests — kiểm thử schema selection and v4/v3 writing.
use super::*;

#[test]
fn web_lock_schema_selection() {
    use crate::lockfile::web_lock_schema_is_v4;
    // Absent config keeps the v3 path (default, byte-identical to before).
    // (Không config giữ đường v3 (mặc định).)
    let bare = tempdir_real().unwrap();
    assert!(!web_lock_schema_is_v4(bare.path()).unwrap());

    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("mgc.toml"),
        "name = \"app\"\necosystem = \"web\"\n[lock]\nschema = \"v4\"\n",
    )
    .unwrap();
    assert!(web_lock_schema_is_v4(dir.path()).unwrap());

    let bad = tempdir_real().unwrap();
    std::fs::write(
        bad.path().join("mgc.toml"),
        "name = \"app\"\necosystem = \"web\"\n[lock]\nschema = \"5\"\n",
    )
    .unwrap();
    assert!(web_lock_schema_is_v4(bad.path()).is_err());
}

#[test]
fn web_writer_emits_v4_with_schema_v4() {
    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("mgc.toml"),
        "name = \"app\"\necosystem = \"web\"\n[lock]\nschema = \"v4\"\n",
    )
    .unwrap();
    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: PackageId::new(
                PackageName::new("stable-web-dep").unwrap(),
                Version::parse("1.0.0").unwrap(),
            ),
            integrity: "sha512-stable".to_string(),
            tarball_url: "https://registry.example/stable-web-dep.tgz".to_string(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };
    crate::lockfile::write_web_lockfile_with_state(dir.path(), &graph, "https://registry.example")
        .unwrap();

    // On-disk version is 4 with a filled digest; the v4 math verifies.
    // (Version trên đĩa là 4 với digest đã điền; toán v4 xác minh được.)
    let path = dir.path().join("mgc.lock");
    let text = std::fs::read_to_string(&path).unwrap();
    assert_eq!(
        mgc_lockfile::migrate::detect_lockfile_version(&text).unwrap(),
        4
    );
    let doc = mgc_lockfile::canonical::parse_v4_document(&text).unwrap();
    assert!(!doc.metadata.lockfile_hash.is_empty());
    assert!(
        mgc_lockfile::policy::verify_v4_math(&doc)
            .unwrap()
            .digest_ok
    );
    // Replay keeps the bytes identical (no rewrite churn).
    // (Replay giữ nguyên byte (không ghi lại thừa).)
    let before = std::fs::read(&path).unwrap();
    crate::lockfile::write_web_lockfile_with_state(dir.path(), &graph, "https://registry.example")
        .unwrap();
    assert_eq!(std::fs::read(&path).unwrap(), before);
}

#[test]
fn web_writer_keeps_v3_without_schema_config() {
    // Same graph, no `[lock] schema`: the v3 path from before, untouched.
    // (Cùng graph, không config schema: đường v3 cũ, giữ nguyên.)
    let dir = tempdir_real().unwrap();
    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: PackageId::new(
                PackageName::new("stable-web-dep").unwrap(),
                Version::parse("1.0.0").unwrap(),
            ),
            integrity: "sha512-stable".to_string(),
            tarball_url: "https://registry.example/stable-web-dep.tgz".to_string(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };
    crate::lockfile::write_web_lockfile_with_state(dir.path(), &graph, "https://registry.example")
        .unwrap();
    let text = std::fs::read_to_string(dir.path().join("mgc.lock")).unwrap();
    assert_eq!(
        mgc_lockfile::migrate::detect_lockfile_version(&text).unwrap(),
        3
    );
}
