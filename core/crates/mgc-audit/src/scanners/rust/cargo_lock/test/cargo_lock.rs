//! Cargo.lock parser regression cases moved from the mixed lane integration target.
//! Test hồi quy parser Cargo.lock được chuyển từ integration target trộn nhiều loại.
#![allow(clippy::unwrap_used)]

use super::*;

#[test]
fn empty_lock_has_no_registry_pins() {
    // An empty lockfile has no third-party registry pins.
    // Lockfile rỗng không có pin registry bên thứ ba.
    let (pins, skipped) = read_cargo_lock_pins("version = 4\n").unwrap();
    assert!(pins.is_empty());
    assert!(skipped.is_empty());
}

#[test]
fn reader_collects_registry_graph_and_marks_non_crates_io_sources() {
    let raw = r#"
version = 4

[[package]]
name = "serde"
version = "1.0.219"
source = "registry+https://github.com/rust-lang/crates.io-index"

[[package]]
name = "private-crate"
version = "2.1.0"
source = "registry+https://packages.example.test/index"

[[package]]
name = "local-crate"
version = "0.1.0"
"#;
    let (pins, skipped) = read_cargo_lock_pins(raw).unwrap();
    assert_eq!(pins.len(), 1);
    assert_eq!(pins[0].name, "serde");
    assert_eq!(pins[0].version, "1.0.219");
    assert_eq!(pins[0].ecosystem, "crates.io");
    assert_eq!(skipped.len(), 1);
    assert!(
        skipped
            .iter()
            .any(|reason| reason.contains("private-crate"))
    );
    assert!(!skipped.iter().any(|reason| reason.contains("local-crate")));
}

#[test]
fn reader_rejects_malformed_toml_and_incomplete_records() {
    assert!(read_cargo_lock_pins("not = [valid").is_err());
    assert!(read_cargo_lock_pins("version = 4\n[[package]]\nname = \"serde\"").is_err());
}
