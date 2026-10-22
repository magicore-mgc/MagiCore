//! Lib language detection tests — kiểm thử nhận diện ngôn ngữ Lib.
//! Owns Rust/TS/Python markers and Unix symlink refusal. — Sở hữu marker Rust/TS/Python và từ chối symlink Unix.

use mgc_lib_adapter::adapter_for;

use super::support::tmp;

#[cfg(unix)]
pub(crate) fn language_detection_does_not_follow_external_mgc_config_symlink() {
    let project = tmp("external-mgc-config-link");
    let external = tmp("external-mgc-config-target");
    std::fs::write(external.join("mgc.toml"), "[lib]\nlanguage = 'go'\n").unwrap();
    std::os::unix::fs::symlink(external.join("mgc.toml"), project.join("mgc.toml")).unwrap();

    assert_eq!(mgc_lib_adapter::detect_language(&project), None);
    let _ = std::fs::remove_dir_all(project);
    let _ = std::fs::remove_dir_all(external);
}

#[cfg(unix)]
pub(crate) fn language_detection_does_not_follow_external_cargo_manifest_symlink() {
    let project = tmp("external-cargo-link");
    let external = tmp("external-cargo-target");
    std::fs::write(
        external.join("Cargo.toml"),
        "[package]\nname='outside'\nversion='1.0.0'\n",
    )
    .unwrap();
    std::os::unix::fs::symlink(external.join("Cargo.toml"), project.join("Cargo.toml")).unwrap();

    assert_eq!(mgc_lib_adapter::detect_language(&project), None);
    let _ = std::fs::remove_dir_all(project);
    let _ = std::fs::remove_dir_all(external);
}

pub(crate) fn detect_rust_project_via_cargo_toml_with_magicore_metadata() {
    let dir = tmp("rust");
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"demo-lib\"\nversion = \"0.1.0\"\n\n[package.metadata.magicore]\ncore = \"lib\"\n\n[dependencies]\nserde = \"1\"\n",
    )
    .unwrap();
    let a = adapter_for(&dir, None, None).unwrap().unwrap();
    assert_eq!(a.language(), "rust");
}

pub(crate) fn detect_ts_project_via_mgc_toml() {
    let dir = tmp("ts");
    std::fs::write(
        dir.join("mgc.toml"),
        "ecosystem = \"lib\"\n\n[lib]\nlanguage = \"ts\"\n",
    )
    .unwrap();
    let a = adapter_for(&dir, None, None).unwrap().unwrap();
    assert_eq!(a.language(), "ts");
}

pub(crate) fn detect_python_project_via_mgc_toml() {
    let dir = tmp("py");
    std::fs::write(
        dir.join("mgc.toml"),
        "ecosystem = \"lib\"\n\n[lib]\nlanguage = \"python\"\n",
    )
    .unwrap();
    let a = adapter_for(&dir, None, None).unwrap().unwrap();
    assert_eq!(a.language(), "python");
}

pub(crate) fn adapter_for_returns_none_for_empty_dir() {
    let dir = tmp("empty");
    assert!(adapter_for(&dir, None, None).unwrap().is_none());
}
