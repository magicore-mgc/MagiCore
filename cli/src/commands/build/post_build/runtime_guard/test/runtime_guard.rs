//! The identity must reach every script and never stack.
//! Danh tính phải tới mọi script và không bao giờ chồng.

use super::super::{BuildIdentity, PREAMBLE_MARKER, inject, preamble, sbom_sha256};

fn identity() -> BuildIdentity {
    BuildIdentity {
        name: "app".to_string(),
        version: "1.0.0".to_string(),
        sbom_sha256: "abc123".to_string(),
    }
}

#[test]
fn the_preamble_names_the_build() {
    let text = preamble(&identity());
    assert!(text.starts_with(PREAMBLE_MARKER));
    assert!(text.contains("globalThis.__MAGICORE_BUILD__"));
    assert!(text.contains("\"sbomSha256\":\"abc123\""));
}

#[test]
fn every_script_receives_the_identity() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("assets")).unwrap();
    std::fs::write(dir.path().join("index.js"), "console.log(1)").unwrap();
    std::fs::write(dir.path().join("assets/chunk.js"), "export{}").unwrap();
    std::fs::write(dir.path().join("style.css"), "body{}").unwrap();

    let touched = inject(dir.path(), &identity()).unwrap();
    assert_eq!(touched, 2, "only scripts, never styles");
    let index = std::fs::read_to_string(dir.path().join("index.js")).unwrap();
    assert!(index.starts_with(PREAMBLE_MARKER));
    assert!(index.contains("console.log(1)"), "the body must survive");
    let css = std::fs::read_to_string(dir.path().join("style.css")).unwrap();
    assert!(!css.contains(PREAMBLE_MARKER));
}

#[test]
fn a_second_injection_replaces_rather_than_stacks() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("app.js"), "console.log(1)").unwrap();
    inject(dir.path(), &identity()).unwrap();
    inject(dir.path(), &identity()).unwrap();

    let text = std::fs::read_to_string(dir.path().join("app.js")).unwrap();
    assert_eq!(
        text.matches(PREAMBLE_MARKER).count(),
        1,
        "a rebuild must not stack preambles: {text}"
    );
}

#[test]
fn the_sbom_hash_is_the_file_hash() {
    let dir = tempfile::tempdir().unwrap();
    let sbom = dir.path().join("sbom.json");
    std::fs::write(&sbom, b"{}").unwrap();
    let hash = sbom_sha256(&sbom).unwrap();
    assert_eq!(hash.len(), 64);
    assert_eq!(
        hash,
        "44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a"
    );
}
