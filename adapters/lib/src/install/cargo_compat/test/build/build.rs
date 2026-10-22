use super::super::*;

#[test]
fn rust_lock_projection_accepts_sibling_cores_but_lib_build_remains_scoped() {
    let mut game_package = locked_package("mgcfixture", &[]);
    game_package.owner_core = Some("game".to_string());
    let lock = Lockfile {
        packages: vec![game_package],
        ..Lockfile::new()
    };

    let projected = rust_packages(&lock).unwrap();
    assert_eq!(projected.len(), 1);
    let error = validate_build_ownership(&projected, "lib").unwrap_err();
    assert!(error.to_string().contains("owned by core 'game'"));

    let lib_package = locked_package("mgcfixture", &[]);
    let lib_lock = Lockfile {
        packages: vec![lib_package],
        ..Lockfile::new()
    };
    validate_build_ownership(&rust_packages(&lib_lock).unwrap(), "lib").unwrap();

    let mut iot_package = locked_package("mgcfixture", &[]);
    iot_package.owner_core = Some("iot".to_string());
    let iot_lock = Lockfile {
        packages: vec![iot_package],
        ..Lockfile::new()
    };
    validate_build_ownership(&rust_packages(&iot_lock).unwrap(), "iot").unwrap();
    let error = validate_build_ownership(&rust_packages(&iot_lock).unwrap(), "lib").unwrap_err();
    assert!(error.to_string().contains("owned by core 'iot'"));
}

#[test]
fn empty_cargo_projection_is_accepted_only_when_already_current() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"empty_demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    std::fs::write(project.path().join(".mgc.core"), "lib\n").unwrap();
    crate::install::write_canonical_lock(project.path(), EcosystemTag::Rust, Vec::new()).unwrap();
    let expected = crate::install::prepare_empty_cargo_lock(project.path(), &[])
        .unwrap()
        .expect("initial empty Cargo.lock projection must be created");
    crate::install::atomic_write_cargo_lock(project.path(), &expected).unwrap();

    assert!(validate_build(project.path()).unwrap().is_none());

    std::fs::write(
        project.path().join("Cargo.lock"),
        "version = 4\n\n[[package]]\nname = \"unlocked_extra\"\nversion = \"1.0.0\"\n",
    )
    .unwrap();
    let error = validate_build(project.path()).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("does not match the MGC dependency graph")
    );
}
