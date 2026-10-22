//! Trust-pending behavior tests — test hành vi của trust pending.

#[test]
fn oversized_package_json_is_rejected_before_full_parse() {
    let temp = tempfile::tempdir().expect("temporary directory must be created");
    let manifest = temp.path().join("package.json");
    let file = std::fs::File::create(&manifest).expect("manifest must be created");
    file.set_len(super::MAX_TRUST_PACKAGE_JSON_BYTES + 1)
        .expect("oversized sparse manifest must be created");

    let result = super::read_bounded_package_json(&manifest);

    assert!(result.is_err(), "oversized manifests must be refused");
}

#[test]
#[cfg(unix)]
fn package_json_symlink_is_not_followed() {
    let temp = tempfile::tempdir().expect("temporary directory must be created");
    let target = temp.path().join("outside.json");
    let link = temp.path().join("package.json");
    std::fs::write(&target, r#"{"scripts":{"install":"evil"}}"#)
        .expect("target manifest must be written");
    std::os::unix::fs::symlink(&target, &link).expect("manifest symlink must be created");

    let result = super::read_bounded_package_json(&link);

    assert!(result.is_err(), "symlinked manifests must be refused");
}

#[test]
fn package_json_directory_is_not_treated_as_missing() {
    let temp = tempfile::tempdir().expect("temporary directory must be created");
    let manifest = temp.path().join("package.json");
    std::fs::create_dir(&manifest).expect("manifest directory must be created");

    let result = super::read_bounded_package_json(&manifest);

    assert!(result.is_err(), "non-regular manifests must be refused");
}
