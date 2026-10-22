//! v4 pin identity, integrity, root selection, and reachability tests.
//! Test identity/integrity pin v4, chọn root và reachability.

#![allow(clippy::unwrap_used)]

/// Write a v4 fixture with its real payload digest.
/// Ghi fixture v4 với digest payload được tính thật.
fn write_v4_fixture(dir: &std::path::Path, text: &str) {
    let mut doc = mgc_lockfile::canonical::parse_v4_document(text).unwrap();
    doc.metadata.lockfile_hash = mgc_lockfile::canonical::payload_digest(&doc.payload());
    let serialized = mgc_lockfile::canonical::write_v4_document(&doc).unwrap();
    std::fs::write(dir.join("mgc.lock"), serialized).unwrap();
}

#[test]
fn v4_pins_come_from_reachable_web_instances_with_jsr_split() {
    // v4 replay: chỉ instance Web tới được từ root vào pins; orphan,
    // non-Web và tên jsr: đi đúng lane của chúng.
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("mgc.toml"),
        "name = \"demo\"\necosystem = \"web\"\n",
    )
    .unwrap();
    std::fs::write(dir.path().join(".mgc.core"), "web\n").unwrap();
    let pin = |ecosystem: &str, name: &str, version: &str| {
        format!(
            "[[package]]\n[package.key]\necosystem = \"{ecosystem}\"\nname = \"{name}\"\nversion = \"{version}\"\nsource_id = \"npm\"\n"
        )
    };
    let edge = |name: &str, version: &str| {
        format!(
            "[[package.edges]]\ntarget_key = {{ ecosystem = \"web\", name = \"{name}\", version = \"{version}\", source_id = \"npm\" }}\nrange = \"*\"\nkind = \"normal\"\norigin = \"Manifest\"\n"
        )
    };
    let mut doc = String::from(
        "version = \"4\"\nroot_dependencies = [\"app@1.0.0\"]\n\n[metadata]\ngenerated_at = \"2026-10-01T00:00:00Z\"\ngenerator = \"mgc/test\"\nlockfile_hash = \"\"\n\n",
    );
    doc.push_str(&pin("web", "app", "1.0.0"));
    doc.push_str(&edge("leftpad", "1.3.0"));
    doc.push_str(&edge("jsr:@scope/pkg", "1.0.0"));
    doc.push_str(&pin("web", "leftpad", "1.3.0"));
    doc.push_str(&pin("web", "jsr:@scope/pkg", "1.0.0"));
    doc.push_str(&pin("web", "orphan", "9.9.9"));
    doc.push_str(&pin("python", "six", "1.17.0"));
    write_v4_fixture(dir.path(), &doc);
    let (pins, jsr_pins) = crate::audit::v4_pins_for_audit(dir.path())
        .unwrap()
        .expect("v4 lock must take the v4 pin path");
    let names: Vec<&str> = pins.iter().map(|(name, _)| name.as_str()).collect();
    assert!(names.contains(&"app"), "root must audit: {names:?}");
    assert!(
        names.contains(&"leftpad"),
        "reachable must audit: {names:?}"
    );
    assert!(
        !names.contains(&"orphan"),
        "orphan must not audit: {names:?}"
    );
    assert!(!names.contains(&"six"), "non-web must not audit: {names:?}");
    assert!(
        !names.iter().any(|name| name.starts_with("jsr:")),
        "{names:?}"
    );
    assert_eq!(jsr_pins, vec!["jsr:@scope/pkg".to_string()]);
}

#[test]
fn v4_audit_derives_roots_from_manifest_when_lock_roots_are_empty() {
    // Web v4 intentionally omits root pins; the manifest supplies anchors.
    // Web v4 cố ý không ghi root pin; manifest cung cấp các điểm neo.
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        r#"{"name":"demo","version":"1.0.0","dependencies":{"app":"1.0.0"}}"#,
    )
    .unwrap();
    std::fs::write(
        dir.path().join("mgc.toml"),
        "name = \"demo\"\necosystem = \"web\"\n",
    )
    .unwrap();
    std::fs::write(dir.path().join(".mgc.core"), "web\n").unwrap();
    let mut doc = String::from(
        "version = \"4\"\nroot_dependencies = []\n\n[metadata]\ngenerated_at = \"2026-10-01T00:00:00Z\"\ngenerator = \"mgc/test\"\nlockfile_hash = \"\"\n\n",
    );
    doc.push_str("[[package]]\n[package.key]\necosystem = \"web\"\nname = \"app\"\nversion = \"1.0.0\"\nsource_id = \"npm\"\n\n");
    doc.push_str("[[package.edges]]\ntarget_key = { ecosystem = \"web\", name = \"leftpad\", version = \"1.3.0\", source_id = \"npm\" }\nrange = \"*\"\nkind = \"normal\"\norigin = \"Manifest\"\n\n");
    doc.push_str("[[package]]\n[package.key]\necosystem = \"web\"\nname = \"leftpad\"\nversion = \"1.3.0\"\nsource_id = \"npm\"\n\n");
    doc.push_str("[[package]]\n[package.key]\necosystem = \"web\"\nname = \"orphan\"\nversion = \"9.9.9\"\nsource_id = \"npm\"\n");
    write_v4_fixture(dir.path(), &doc);

    let (pins, jsr_pins) = crate::audit::v4_pins_for_audit(dir.path())
        .unwrap()
        .expect("v4 lock must take the v4 pin path");
    let names: Vec<&str> = pins.iter().map(|(name, _)| name.as_str()).collect();
    assert_eq!(names, vec!["app", "leftpad"]);
    assert!(jsr_pins.is_empty());
}

#[test]
fn v4_audit_explicit_empty_owner_does_not_query_sibling_pins() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("mgc.toml"),
        "name = \"web\"\necosystem = \"web\"\n",
    )
    .unwrap();
    std::fs::write(dir.path().join(".mgc.core"), "web\n").unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        r#"{"name":"web","dependencies":{}}"#,
    )
    .unwrap();
    let text = "version = \"4\"\nroot_dependencies = [\"sibling@1.0.0\"]\n[root_dependencies_by_owner]\nweb = []\nlib = [\"sibling@1.0.0\"]\n[metadata]\ngenerated_at = \"2026-10-01T00:00:00Z\"\ngenerator = \"mgc/test\"\nlockfile_hash = \"\"\n[[package]]\n[package.key]\necosystem = \"web\"\nname = \"sibling\"\nversion = \"1.0.0\"\nsource_id = \"npm\"\n";
    write_v4_fixture(dir.path(), text);
    let (pins, jsr) = crate::audit::v4_pins_for_audit(dir.path())
        .unwrap()
        .unwrap();
    assert!(
        pins.is_empty() && jsr.is_empty(),
        "empty Web owner must not audit Lib pins"
    );
}

#[test]
fn v4_audit_rejects_payload_tampering_and_unresolved_full_edge_keys() {
    // A valid digest is required before audit; edge identity includes the
    // complete source/variant key, not only name and version.
    // Digest hợp lệ là điều kiện bắt buộc; cạnh dùng đủ khóa source/variant.
    let tampered = tempfile::tempdir().unwrap();
    let valid = "version = \"4\"\n\n[metadata]\ngenerated_at = \"2026-10-01T00:00:00Z\"\ngenerator = \"mgc/test\"\nlockfile_hash = \"\"\n";
    write_v4_fixture(tampered.path(), valid);
    let lock_path = tampered.path().join("mgc.lock");
    let contents = std::fs::read_to_string(&lock_path).unwrap();
    std::fs::write(
        &lock_path,
        contents.replace("generator = \"mgc/test\"", "generator = \"mgc/forged\""),
    )
    .unwrap();
    let err = crate::audit::v4_pins_for_audit(tampered.path()).unwrap_err();
    assert!(err.to_string().contains("payload digest"), "{err}");

    let broken_edge = tempfile::tempdir().unwrap();
    std::fs::write(
        broken_edge.path().join("mgc.toml"),
        "name = \"demo\"\necosystem = \"web\"\n",
    )
    .unwrap();
    std::fs::write(broken_edge.path().join(".mgc.core"), "web\n").unwrap();
    let mut doc = String::from(
        "version = \"4\"\nroot_dependencies = [\"app@1.0.0\"]\n\n[metadata]\ngenerated_at = \"2026-10-01T00:00:00Z\"\ngenerator = \"mgc/test\"\nlockfile_hash = \"\"\n\n",
    );
    doc.push_str("[[package]]\n[package.key]\necosystem = \"web\"\nname = \"app\"\nversion = \"1.0.0\"\nsource_id = \"npm\"\n");
    doc.push_str("\n[[package.edges]]\ntarget_key = { ecosystem = \"web\", name = \"leftpad\", version = \"1.3.0\", source_id = \"mirror\" }\nrange = \"*\"\nkind = \"normal\"\norigin = \"Manifest\"\n\n");
    doc.push_str("[[package]]\n[package.key]\necosystem = \"web\"\nname = \"leftpad\"\nversion = \"1.3.0\"\nsource_id = \"npm\"\n");
    write_v4_fixture(broken_edge.path(), &doc);
    let err = crate::audit::v4_pins_for_audit(broken_edge.path()).unwrap_err();
    assert!(err.to_string().contains("edge target"), "{err}");
}

#[test]
fn v4_pins_returns_none_for_legacy_locks() {
    // Lock legacy → None để đường legacy cũ sở hữu (không tranh).
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("mgc.lock"), "version = \"3\"\n").unwrap();
    assert!(
        crate::audit::v4_pins_for_audit(dir.path())
            .unwrap()
            .is_none()
    );
    let empty = tempfile::tempdir().unwrap();
    assert!(
        crate::audit::v4_pins_for_audit(empty.path())
            .unwrap()
            .is_none()
    );
}
