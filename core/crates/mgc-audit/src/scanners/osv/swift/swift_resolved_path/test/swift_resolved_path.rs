//! Tests for bounded Package.resolved lookup.
//! Kiểm thử phạm vi tìm Package.resolved có giới hạn.

#![allow(clippy::unwrap_used)]

use super::*;

fn tmp(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("mgc-detect-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// REVIEW F1: the swift locator scope — root, ios/, exactly one
/// parent. Anything farther is another scope's lockfile.
/// Scope locator swift — root, ios/, đúng một cấp cha.
#[test]
fn swift_resolved_path_scope_is_bounded() {
    let lock = r#"{"pins":[],"version":2}"#;

    let root_hit = tmp("swift-root");
    std::fs::write(root_hit.join("Package.resolved"), lock).unwrap();
    assert_eq!(
        swift_resolved_path(&root_hit).as_deref(),
        Some(root_hit.join("Package.resolved").as_path())
    );

    let rn = tmp("swift-rn");
    std::fs::create_dir_all(rn.join("ios")).unwrap();
    std::fs::write(rn.join("ios").join("Package.resolved"), lock).unwrap();
    assert!(swift_resolved_path(&rn).is_some());

    let outer = tmp("swift-outer");
    std::fs::write(outer.join("Package.resolved"), lock).unwrap();
    let mid = outer.join("mid");
    std::fs::create_dir_all(&mid).unwrap();
    assert!(
        swift_resolved_path(&mid).is_some(),
        "one parent level is in scope"
    );
    let deep = mid.join("deep");
    std::fs::create_dir_all(&deep).unwrap();
    assert!(
        swift_resolved_path(&deep).is_none(),
        "two levels up is another scope"
    );

    let bare = tmp("swift-bare");
    assert!(swift_resolved_path(&bare).is_none());
}
