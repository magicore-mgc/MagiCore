#![allow(clippy::unwrap_used)]

use super::super::*;

#[test]
fn rejects_empty_or_whitespace_tokens_without_creating_a_file() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join(".npmrc");
    assert!(NpmRc::save_auth_token(&path, "registry.example", "").is_err());
    assert!(NpmRc::save_auth_token(&path, "registry.example", "bad token").is_err());
    assert!(!path.exists());

    // A loopback registry stays usable: it is a real local target.
    // (Registry loopback vẫn dùng được: đó là đích local thật.)
    NpmRc::save_auth_token_for_registry_url(&path, "http://localhost:4315/", "secret")
        .expect("a loopback registry is a valid scope");
}

#[cfg(unix)]
#[test]
fn refuses_symlink_target_and_writes_new_file_with_private_mode() {
    use std::os::unix::fs::{PermissionsExt, symlink};

    let directory = tempfile::tempdir().unwrap();
    let target = directory.path().join("target");
    std::fs::write(&target, "registry=https://keep.example/\n").unwrap();
    let link = directory.path().join(".npmrc-link");
    symlink(&target, &link).unwrap();
    assert!(NpmRc::save_auth_token(&link, "registry.example", "fixture-token").is_err());
    assert_eq!(
        std::fs::read_to_string(&target).unwrap(),
        "registry=https://keep.example/\n"
    );

    let path = directory.path().join(".npmrc");
    NpmRc::save_auth_token_for_registry_url(
        &path,
        "https://registry.example/team",
        "fixture-token",
    )
    .unwrap();
    let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
    assert_eq!(mode, 0o600);
}

/// SEC-13: a registry URL whose host has no dot would produce a scope such as
/// `//:_authToken`, which npm applies to every registry.
/// (SEC-13: URL registry có host không dấu chấm sẽ sinh scope dạng
/// `//:_authToken`, npm áp cho mọi registry.)
#[test]
fn refuses_hostless_auth_token_scope() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join(".npmrc");
    for registry in ["https:///", "http://", "https://intranet/"] {
        let error = NpmRc::save_auth_token_for_registry_url(&path, registry, "secret")
            .err()
            .unwrap_or_else(|| panic!("{registry} must be refused"));
        assert!(
            error.to_string().contains("dotted or loopback host")
                || error.to_string().contains("invalid registry URL"),
            "{registry}: {error}"
        );
    }
    assert!(!path.exists());
}
