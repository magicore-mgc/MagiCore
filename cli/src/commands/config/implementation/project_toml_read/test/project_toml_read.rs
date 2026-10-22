use super::super::super::test_support::temp_dir_with_mgc_toml;
use super::super::*;
use std::fs;

#[test]
fn toml_value_reads_top_level_key() {
    let dir =
        temp_dir_with_mgc_toml("name = \"myapp\"\necosystem = \"web\"\nversion = \"0.1.0\"\n");
    let val = toml_value(&dir.path().join("mgc.toml"), "ecosystem");
    assert_eq!(val.as_deref(), Some("web"));
    let ver = toml_value(&dir.path().join("mgc.toml"), "version");
    assert_eq!(ver.as_deref(), Some("0.1.0"));
}

#[test]
fn toml_value_reads_dot_notation() {
    let dir =
        temp_dir_with_mgc_toml("[game]\nengine = \"bevy\"\n\n[iot]\nframework = \"esp-idf\"\n");
    let val = toml_value(&dir.path().join("mgc.toml"), "game.engine");
    assert_eq!(val.as_deref(), Some("bevy"));
    let iot_val = toml_value(&dir.path().join("mgc.toml"), "iot.framework");
    assert_eq!(iot_val.as_deref(), Some("esp-idf"));
}

#[test]
fn config_get_redacts_secret_keys_and_nested_parent_tables() {
    let dir = temp_dir_with_mgc_toml(
        "[registry]\ntoken = \"secret-token\"\npassword = \"secret-password\"\nurl = \"https://registry.example\"\n",
    );
    let path = dir.path().join("mgc.toml");

    let token = toml_value(&path, "registry.token").expect("token key exists");
    let registry = toml_value(&path, "registry").expect("registry table exists");

    assert_eq!(token, "***");
    assert!(registry.contains("token = \"***\""));
    assert!(registry.contains("password = \"***\""));
    assert!(registry.contains("https://registry.example"));
    assert!(!registry.contains("secret-token"));
    assert!(!registry.contains("secret-password"));
}

#[cfg(unix)]
#[test]
fn toml_value_refuses_symlinked_config() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("outside.toml");
    let path = dir.path().join("mgc.toml");
    fs::write(&target, "name = \"outside-secret\"\n").unwrap();
    symlink(&target, &path).unwrap();

    assert_eq!(toml_value(&path, "name"), None);
}
