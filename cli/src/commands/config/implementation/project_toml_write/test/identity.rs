use super::super::super::test_support::temp_dir_with_mgc_toml;
use super::super::*;
use std::fs;

#[test]
fn set_toml_cannot_reassign_core_identity_and_preserves_file() {
    let dir = temp_dir_with_mgc_toml("name = \"sample\"\necosystem = \"web\"\n");
    let path = dir.path().join("mgc.toml");
    let before = fs::read(&path).unwrap();

    let err = set_toml_at(&path, "ecosystem", "ai").unwrap_err();

    assert!(err.to_string().contains("mgc init --signature"));
    assert_eq!(fs::read(&path).unwrap(), before);
}

#[test]
fn delete_toml_cannot_remove_core_identity_and_preserves_file() {
    let dir = temp_dir_with_mgc_toml("name = \"sample\"\necosystem = \"web\"\n");
    let path = dir.path().join("mgc.toml");
    let before = fs::read(&path).unwrap();

    let err = delete_toml_at(&path, "ecosystem").unwrap_err();

    assert!(err.to_string().contains("mgc init --signature"));
    assert_eq!(fs::read(&path).unwrap(), before);
}

#[test]
fn config_cannot_write_nested_ecosystem_keys() {
    let dir = temp_dir_with_mgc_toml("name = \"sample\"\necosystem = \"web\"\n");
    let path = dir.path().join("mgc.toml");
    let before = fs::read(&path).unwrap();

    let err = set_toml_at(&path, "ecosystem.core", "ai").unwrap_err();

    assert!(err.to_string().contains("mgc init --signature"));
    assert_eq!(fs::read(&path).unwrap(), before);
}
