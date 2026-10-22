#[test]
fn dev_scheme_reads_mgc_toml() {
    let dir = std::env::temp_dir().join(format!("mgc-app-scheme-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    assert!(super::dev_scheme(&dir).is_none());
    std::fs::write(
        dir.join("mgc.toml"),
        "[app]\nlanguage = \"objc\"\ndev_scheme = \"App\"\n",
    )
    .unwrap();
    assert_eq!(super::dev_scheme(&dir).as_deref(), Some("App"));
    std::fs::write(dir.join("mgc.toml"), "[app]\nlanguage = \"objc\"\n").unwrap();
    assert!(super::dev_scheme(&dir).is_none());
    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn app_dev_settings_ignore_external_mgc_toml_symlink() {
    let root = tempfile::tempdir().unwrap();
    let external = tempfile::tempdir().unwrap();
    std::fs::write(
        external.path().join("mgc.toml"),
        "[app]\ndev_scheme = 'External'\napplication_id = 'com.external.app'\n",
    )
    .unwrap();
    std::os::unix::fs::symlink(
        external.path().join("mgc.toml"),
        root.path().join("mgc.toml"),
    )
    .unwrap();

    assert_eq!(super::dev_scheme(root.path()), None);
    assert_eq!(super::read_app_id(root.path()), None);
}
