//! Manifest round-trip tests for the shared JS/npm service.
//! Test vòng lặp manifest cho service JS/npm dùng chung.
//!
//! These tests pin the canonical `parse_manifest`/`write_manifest` behavior at
//! its new owner (moved verbatim from `mgc-web-adapter` in S07). The web facade
//! delegates here, so this suite is the nearest-owner proof for the shared
//! implementation; web's own suites cover the facade path.
//! Các test này ghim behavior chuẩn của `parse_manifest`/`write_manifest` tại
//! owner mới (chuyển nguyên văn từ `mgc-web-adapter` trong S07). Facade web ủy
//! quyền về đây nên đây là proof gần owner nhất cho implementation dùng chung;
//! suite của web phủ đường facade.

use mgc_js_service::manifest::{parse_manifest, write_manifest};

/// Canonicalize the temp root: macOS mounts /tmp via a /var → /private/var
/// symlink and containment checks compare canonical paths.
/// Chuẩn hóa temp root: macOS mount /tmp qua symlink /var → /private/var nên
/// kiểm tra containment phải so sánh path đã canonical.
fn real_root(dir: &tempfile::TempDir) -> std::path::PathBuf {
    dir.path()
        .canonicalize()
        .expect("canonicalize temp project root")
}

#[test]
fn manifest_writer_preserves_workspace_protocol_dependencies() {
    let dir = tempfile::tempdir().expect("temp project");
    let root = real_root(&dir);
    std::fs::write(
        root.join("package.json"),
        serde_json::json!({
            "name": "frontend",
            "version": "0.1.0",
            "dependencies": {
                "@core/shared": "workspace:*",
                "react": "^18.2.0"
            }
        })
        .to_string(),
    )
    .expect("write package.json");

    let manifest = parse_manifest(&root).expect("parse package.json");
    write_manifest(&root, &manifest).expect("rewrite package.json");

    let package: serde_json::Value =
        serde_json::from_slice(&std::fs::read(root.join("package.json")).expect("read back"))
            .expect("valid JSON");
    assert_eq!(
        package["dependencies"]["@core/shared"], "workspace:*",
        "shared-service writes must preserve workspace protocol entries outside its registry graph"
    );
    assert_eq!(
        package["dependencies"]["react"], "^18.2.0",
        "shared-service writes must preserve registry ranges verbatim"
    );
}
