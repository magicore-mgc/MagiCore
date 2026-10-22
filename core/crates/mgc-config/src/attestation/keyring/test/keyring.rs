#![allow(clippy::unwrap_used)]

use super::super::*;

#[test]
fn malformed_existing_keyring_is_not_replaced() {
    let directory = tempfile::tempdir().expect("keyring directory");
    let path = directory.path().join("keyring.json");
    std::fs::write(&path, b"{").expect("write malformed keyring");
    assert!(load_or_create_keyring(&path).is_err());
    assert_eq!(std::fs::read(&path).expect("read keyring"), b"{");
}
