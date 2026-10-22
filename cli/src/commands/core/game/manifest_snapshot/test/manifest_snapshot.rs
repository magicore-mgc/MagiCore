//! Preserve the typed snapshot failure and its I/O source chain.
//! Giữ lỗi snapshot typed và source chain I/O.
use super::super::read_manifest_snapshot;
use crate::commands::core::game::error::GameOptimizerError;
use std::error::Error as StdError;

#[test]
fn optimizer_manifest_snapshot_failure_is_typed_and_preserves_io_source() {
    let dir = tempfile::tempdir().unwrap();
    let manifest = dir.path().join("Cargo.toml");
    let error = read_manifest_snapshot(&manifest).unwrap_err();
    let typed = error
        .downcast_ref::<GameOptimizerError>()
        .expect("typed snapshot error");

    assert!(typed.to_string().contains("cannot snapshot Cargo.toml"));
    let source = StdError::source(typed)
        .expect("snapshot I/O source")
        .downcast_ref::<std::io::Error>()
        .expect("native I/O error");
    assert_eq!(source.kind(), std::io::ErrorKind::NotFound);
    assert!(!dir.path().join(".magicore").exists());
}
