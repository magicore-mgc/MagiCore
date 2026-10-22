use super::super::super::test_support::temp_dir_with_mgc_toml;
use super::super::*;
use std::fs;

#[test]
fn project_toml_script_policy_edits_refuse_to_race_an_install() {
    use std::time::Duration;

    let dir = temp_dir_with_mgc_toml(
        "name = \"test-project\"\necosystem = \"web\"\nversion = \"0.1.0\"\n[lock]\nacquire_timeout_ms = 1\n[scripts]\ndeny = [\"sample-package\"]\n",
    );
    let path = dir.path().join("mgc.toml");
    let before = fs::read(&path).expect("original project config must be readable");
    let _held =
        mgc_lockfile::project_lock::ProjectWriteLock::acquire(dir.path(), Duration::from_secs(1))
            .expect("dependency mutation lock must be acquired");

    let set_result = set_toml_at(&path, "scripts.allow", "[\"sample-package\"]");
    let delete_result = delete_toml_at(&path, "scripts.deny");

    assert!(set_result.is_err(), "TOML set must not race install");
    assert!(delete_result.is_err(), "TOML delete must not race install");
    assert_eq!(
        fs::read(path).expect("project config must remain readable"),
        before,
        "a blocked policy edit must leave the project config byte-identical"
    );
}

#[test]
fn set_toml_still_allows_non_identity_project_settings() {
    let dir = temp_dir_with_mgc_toml("name = \"sample\"\necosystem = \"web\"\n");
    let path = dir.path().join("mgc.toml");

    set_toml_at(&path, "build.target", "release").unwrap();

    let content = fs::read_to_string(path).unwrap();
    assert!(content.contains("ecosystem = \"web\""));
    assert!(content.contains("target = \"release\""));
}

#[cfg(unix)]
#[test]
fn set_toml_preserves_private_file_mode() {
    use std::os::unix::fs::PermissionsExt;

    let dir = temp_dir_with_mgc_toml("name = \"sample\"\necosystem = \"web\"\n");
    let path = dir.path().join("mgc.toml");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();

    set_toml_at(&path, "name", "updated").unwrap();

    assert_eq!(
        fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o600
    );
}
