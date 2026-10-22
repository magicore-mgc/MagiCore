//! v4 lane merge tests / Test gộp lane v4.
#![allow(clippy::unwrap_used)]

use crate::install::lockfile::test::fixtures::{package, v4_package, write_mgc_toml_schema};
use crate::install::lockfile::write_canonical_lock;
use mgc_lockfile::EcosystemTag;

#[test]
fn v4_schema_writes_v4_lock() {
    let dir = tempfile::tempdir().unwrap();
    write_mgc_toml_schema(dir.path(), "v4");
    write_canonical_lock(
        dir.path(),
        &[EcosystemTag::Dart],
        vec![v4_package("http", "1.0.0")],
    )
    .unwrap();
    let text = std::fs::read_to_string(dir.path().join("mgc.lock")).unwrap();
    assert_eq!(
        mgc_lockfile::migrate::detect_lockfile_version(&text).unwrap(),
        4
    );
    let doc = mgc_lockfile::canonical::parse_v4_document(&text).unwrap();
    assert!(
        mgc_lockfile::policy::verify_v4_math(&doc)
            .unwrap()
            .digest_ok
    );
    assert!(doc.packages.iter().any(|p| p.key.name == "http"));
}

#[test]
fn v4_schema_refuses_lossy_lane_rows() {
    // Rows without registry/SRI cannot become v4: the v3 lock stays
    // untouched and the refusal names the gap (never a partial v4).
    // (Row thiếu registry/SRI không thành v4 được: giữ nguyên lock v3 và
    // từ chối nêu rõ khoảng trống (không v4 nửa vời).)
    let dir = tempfile::tempdir().unwrap();
    write_mgc_toml_schema(dir.path(), "v4");
    let err = write_canonical_lock(
        dir.path(),
        &[EcosystemTag::Dart],
        vec![package("bare", "1.0.0", EcosystemTag::Dart)],
    )
    .unwrap_err();
    assert!(
        err.to_string().contains("lossy"),
        "expected lossy refusal, got: {err}"
    );
    assert!(
        !dir.path().join("mgc.lock").exists(),
        "refused v4 write must not publish any lock"
    );
}

#[test]
fn v4_replay_keeps_bytes_identical() {
    let dir = tempfile::tempdir().unwrap();
    write_mgc_toml_schema(dir.path(), "v4");
    let packages = vec![v4_package("http", "1.0.0")];
    write_canonical_lock(dir.path(), &[EcosystemTag::Dart], packages.clone()).unwrap();
    let before = std::fs::read(dir.path().join("mgc.lock")).unwrap();
    write_canonical_lock(dir.path(), &[EcosystemTag::Dart], packages).unwrap();
    assert_eq!(std::fs::read(dir.path().join("mgc.lock")).unwrap(), before);
}
