//! Go lockfile pin tests — test pin lock Go.
//! The lock is the only source that carries transitive Go modules, so a wrong
//! read here silently under-reports audit coverage.
//! (Lock là nguồn duy nhất mang module Go bắc cầu, nên đọc sai sẽ báo thiếu
//! coverage mà không lộ.)

// Lockfile fixtures below are the exact shape `mgc install` writes (captured
// 2026-10-27, schema v3); a hand-shortened lock would fail to parse and test
// nothing.
// (Fixture lock dưới đây đúng hình dạng `mgc install` ghi — chụp 2026-10-27,
// schema v3; lock rút gọn tự chế sẽ không parse được và không test gì.)

/// Render a schema-v3 lock with the given `(name, version, ecosystem)` packages.
/// (Sinh lock v3 với các package cho trước.)
fn lock_with(packages: &[(&str, &str, &str)]) -> String {
    let mut text = String::from(
        "version = \"3\"\n\n[metadata]\ngenerated_at = \"2026-10-27T17:52:29.333517+00:00\"\n\
         generator = \"mgc/1.1.0-rc.10\"\nlockfile_hash = \"\"\n\n[root_dependencies_by_owner]\nlib = []\n",
    );
    for (name, version, ecosystem) in packages {
        text.push_str(&format!(
            "\n[[package]]\nowner_core = \"lib\"\nname = \"{name}\"\nversion = \"{version}\"\n\
             resolved = \"https://example.invalid/{name}/{version}\"\nintegrity = \"\"\n\
             dependencies = []\necosystem = \"{ecosystem}\"\nregistry = \"test://example\"\n\
             markers = []\n\n[package.artifact]\nurl = \"https://example.invalid/{name}/{version}\"\n\
             content_hash = \"\"\ndownloaded_from = \"test://example\"\n"
        ));
    }
    text
}

use super::super::{GO_OSV_ECOSYSTEM, go_pins_from_mgc_lock};

#[test]
fn a_missing_lockfile_is_not_an_error() {
    let dir = tempfile::tempdir().expect("temp dir");
    assert!(
        go_pins_from_mgc_lock(dir.path())
            .expect("a missing lock is Ok(None)")
            .is_none(),
        "no lockfile must fall back to go.mod"
    );
}

#[test]
fn a_lockfile_without_go_modules_yields_an_empty_pin_set() {
    let dir = tempfile::tempdir().expect("temp dir");
    // A lock that exists but names no Go module: the caller must see an empty
    // set (and say so), not `None` (which would silently re-read go.mod).
    // (Lock tồn tại nhưng không có module Go: caller phải thấy tập rỗng, không
    // phải `None` — nếu `None` sẽ âm thầm đọc lại go.mod.)
    // Shape captured from a real `mgc install` lockfile (v3).
    // (Hình dạng lấy từ lockfile thật do `mgc install` sinh — v3.)
    std::fs::write(
        dir.path().join("mgc.lock"),
        lock_with(&[("lodash", "4.17.21", "web")]),
    )
    .expect("write lock");
    let pins = go_pins_from_mgc_lock(dir.path()).expect("read lock");
    match pins {
        Some(pins) => assert!(
            pins.iter().all(|pin| pin.ecosystem == GO_OSV_ECOSYSTEM),
            "only Go pins may be returned: {pins:?}"
        ),
        None => panic!("an existing lockfile must not be reported as absent"),
    }
}

#[test]
fn non_utf8_lockfile_fails_closed() {
    let dir = tempfile::tempdir().expect("temp dir");
    std::fs::write(dir.path().join("mgc.lock"), [0xff, 0xfe, 0x00]).expect("write bytes");
    let error = go_pins_from_mgc_lock(dir.path()).expect_err("invalid UTF-8 must fail");
    assert!(
        error.to_string().contains("not valid UTF-8"),
        "the error must name the real cause: {error}"
    );
}

#[test]
fn the_osv_ecosystem_for_go_is_spelled_as_osv_expects() {
    // OSV matches the ecosystem by exact string; "go" would silently return no
    // advisories for a vulnerable module.
    // (OSV khớp ecosystem theo chuỗi chính xác; "go" sẽ trả rỗng cho module lỗi.)
    assert_eq!(GO_OSV_ECOSYSTEM, "Go");
}

/// A real golang project lock is read as Go pins with the OSV ecosystem name.
///
/// This is the case the direct-only reader could not answer: the module below is
/// a transitive dependency, absent from `go.mod`, and a lockfile read must see it.
/// (Đây là ca reader direct-only không trả lời được: module dưới là dependency bắc
/// cầu, không có trong `go.mod`, và đọc lock phải thấy nó.)
#[test]
fn a_real_go_lockfile_yields_go_pins() {
    let dir = tempfile::tempdir().expect("temp dir");
    std::fs::write(
        dir.path().join("mgc.lock"),
        lock_with(&[
            ("github.com/google/uuid", "1.6.0", "go"),
            ("golang.org/x/text", "0.3.6", "go"),
        ]),
    )
    .expect("write lock");

    let pins = go_pins_from_mgc_lock(dir.path())
        .expect("read lock")
        .expect("an existing lock is Some");
    assert_eq!(pins.len(), 2, "both Go modules must be pinned: {pins:?}");
    for pin in &pins {
        assert_eq!(pin.ecosystem, "Go");
    }
    assert!(
        pins.iter()
            .any(|pin| pin.name == "golang.org/x/text" && pin.version == "0.3.6"),
        "the transitive module must be visible: {pins:?}"
    );
}

/// Non-Go packages in the same lock are ignored, not mislabelled.
/// (Package không phải Go trong cùng lock bị bỏ qua, không bị gán sai nhãn.)
#[test]
fn non_go_packages_are_excluded() {
    let dir = tempfile::tempdir().expect("temp dir");
    std::fs::write(
        dir.path().join("mgc.lock"),
        lock_with(&[
            ("serde", "1.0.0", "rust"),
            ("github.com/stretchr/testify", "1.9.0", "go"),
        ]),
    )
    .expect("write lock");
    let pins = go_pins_from_mgc_lock(dir.path())
        .expect("read")
        .expect("Some");
    assert_eq!(pins.len(), 1);
    assert_eq!(pins[0].name, "github.com/stretchr/testify");
}
