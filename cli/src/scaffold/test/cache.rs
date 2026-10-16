//! Versioned cache compatibility API regressions using isolated roots.
//! Kiểm cache cũ bằng archive thật; không đổi environment toàn process.
use super::*;
use crate::scaffold::spec::{CoreKind, ScaffoldRef};

fn spec(core: CoreKind) -> ScaffoldSpec {
    ScaffoldSpec {
        core,
        name: "fixture".to_string(),
        normalized_name: "fixture".to_string(),
        requested_ref: ScaffoldRef::Default,
    }
}

fn archive() -> Vec<u8> {
    let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut tar = tar::Builder::new(encoder);
    for (path, body) in [
        (
            "package/template.toml",
            b"[[files]]\nsource = 'main.txt'\ntarget = 'main.txt'\n".as_slice(),
        ),
        ("package/sources/main.txt", b"fixture content".as_slice()),
    ] {
        let mut header = tar::Header::new_gnu();
        header.set_mode(0o644);
        header.set_size(body.len() as u64);
        header.set_cksum();
        tar.append_data(&mut header, path, body).unwrap();
    }
    tar.into_inner().unwrap().finish().unwrap()
}

#[test]
fn explicit_cache_root_overrides_the_platform_default() {
    assert_eq!(
        cache_root_with_override(Some(PathBuf::from("explicit")), PathBuf::from("default")),
        PathBuf::from("explicit")
    );
    assert_eq!(
        cache_root_with_override(None, PathBuf::from("default")),
        PathBuf::from("default")
    );
}

#[test]
fn four_core_cache_writes_publish_complete_contracts_and_semantic_versions() {
    for core in [CoreKind::Web, CoreKind::Ai, CoreKind::App, CoreKind::Lib] {
        let root = tempfile::tempdir().unwrap();
        let root = root.path().canonicalize().unwrap();
        let spec = spec(core);
        for version in ["2.0.0", "10.0.0"] {
            write_at(&root, &spec, version, &archive()).unwrap();
            let entry = validated_entry(&root, &spec, version).unwrap().unwrap();
            assert_eq!(
                fs::read(entry.join("sources/main.txt")).unwrap(),
                b"fixture content"
            );
            assert_eq!(
                read_template_cache_version(&entry).unwrap().as_deref(),
                Some(version)
            );
        }
        assert_eq!(list_versions_at(&root, &spec), ["10.0.0", "2.0.0"]);
        // A valid cache hit must not parse or replace it using a later bad download.
        // Cache đã hoàn tất không bị ghi đè khi caller truyền archive lỗi.
        write_at(&root, &spec, "10.0.0", b"invalid later archive").unwrap();
        assert!(validated_entry(&root, &spec, "10.0.0").unwrap().is_some());
    }
}

#[test]
fn partial_or_nested_contracts_are_not_cache_hits() {
    let root = tempfile::tempdir().unwrap();
    let root = root.path().canonicalize().unwrap();
    let spec = spec(CoreKind::Ai);
    let target = root.join("ai/fixture/1.0.0");
    fs::create_dir_all(target.join("nested/sources")).unwrap();
    fs::write(target.join("nested/template.toml"), "files = []").unwrap();
    fs::write(target.join(".mgc-version"), "1.0.0").unwrap();
    assert!(validated_entry(&root, &spec, "1.0.0").unwrap().is_none());
    assert!(list_versions_at(&root, &spec).is_empty());
    fs::write(target.join("sentinel"), "preserve").unwrap();
    assert!(write_at(&root, &spec, "1.0.0", b"invalid archive").is_err());
    assert_eq!(fs::read(target.join("sentinel")).unwrap(), b"preserve");
    write_at(&root, &spec, "1.0.0", &archive()).unwrap();
    assert!(validated_entry(&root, &spec, "1.0.0").unwrap().is_some());
    assert!(!target.join("sentinel").exists());
}

#[test]
fn cache_identity_and_version_cannot_escape_the_selected_root() {
    let root = tempfile::tempdir().unwrap();
    let mut spec = spec(CoreKind::Lib);
    for name in [
        "../escape",
        "a/../escape",
        "/absolute",
        "C:/drive",
        "a\\escape",
        "a/.. /escape",
    ] {
        spec.normalized_name = name.to_string();
        assert!(write_at(root.path(), &spec, "1.0.0", &archive()).is_err());
    }
    spec.normalized_name = "fixture".to_string();
    for version in ["../escape", "latest", "1.0", ""] {
        assert!(write_at(root.path(), &spec, version, &archive()).is_err());
    }
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
}

#[cfg(unix)]
#[test]
fn versioned_cache_rejects_linked_ancestors_without_touching_their_target() {
    let root = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    fs::write(outside.path().join("sentinel"), "preserve").unwrap();
    std::os::unix::fs::symlink(outside.path(), root.path().join("app")).unwrap();
    let spec = spec(CoreKind::App);
    assert!(write_at(root.path(), &spec, "1.0.0", &archive()).is_err());
    assert!(validated_entry(root.path(), &spec, "1.0.0").is_err());
    assert_eq!(fs::read_dir(outside.path()).unwrap().count(), 1);
}

#[test]
fn clearing_one_version_or_layer_preserves_other_versions_and_cores() {
    let root = tempfile::tempdir().unwrap();
    let root = root.path().canonicalize().unwrap();
    let a = spec(CoreKind::Web);
    let b = spec(CoreKind::Ai);
    for (spec, version) in [(&a, "1.0.0"), (&a, "2.0.0"), (&b, "1.0.0")] {
        write_at(&root, spec, version, &archive()).unwrap();
    }
    clear_at(&root, &a, Some("1.0.0")).unwrap();
    assert_eq!(list_versions_at(&root, &a), ["2.0.0"]);
    assert!(validated_entry(&root, &b, "1.0.0").unwrap().is_some());
    clear_at(&root, &a, None).unwrap();
    assert!(list_versions_at(&root, &a).is_empty());
    assert!(validated_entry(&root, &b, "1.0.0").unwrap().is_some());
    clear_at(&root, &a, None).unwrap();
}

#[test]
fn invalid_clear_requests_do_not_remove_sibling_data() {
    let root = tempfile::tempdir().unwrap();
    let root = root.path().canonicalize().unwrap();
    let mut spec = spec(CoreKind::Lib);
    fs::write(root.join("sentinel"), "preserve").unwrap();
    assert!(clear_at(&root, &spec, Some("../escape")).is_err());
    spec.normalized_name = "..".to_string();
    assert!(clear_at(&root, &spec, None).is_err());
    assert_eq!(fs::read(root.join("sentinel")).unwrap(), b"preserve");
}

#[test]
fn clearing_absent_entries_does_not_create_empty_cache_directories_or_locks() {
    let root = tempfile::tempdir().unwrap();
    let spec = spec(CoreKind::App);
    clear_at(root.path(), &spec, Some("1.0.0")).unwrap();
    clear_at(root.path(), &spec, None).unwrap();
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
}
