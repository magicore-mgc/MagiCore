//! Trust-pending behavior tests — test hành vi của trust pending.

#[test]
fn unreadable_node_modules_tree_is_not_reported_as_no_scripted_packages() {
    let temp = tempfile::tempdir().expect("temporary directory must be created");
    let blocker = temp.path().join("not-a-directory");
    std::fs::write(&blocker, "not a directory").expect("blocker must be written");

    let result = super::scan_scripted_packages(&blocker);

    assert!(
        result.is_err(),
        "an unreadable package tree must not look like an empty tree"
    );
}

#[test]
fn recursive_scan_finds_scoped_and_nested_lifecycle_packages() {
    let temp = tempfile::tempdir().expect("temporary directory must be created");
    let node_modules = temp.path().join("node_modules");
    let scoped_package = node_modules.join("@scope/package");
    let nested_package = scoped_package.join("node_modules/child-package");
    std::fs::create_dir_all(&nested_package).expect("package tree must be created");
    std::fs::write(
        scoped_package.join("package.json"),
        r#"{"name":"@scope/package","version":"1.0.0","scripts":{"install":"node setup.js"}}"#,
    )
    .expect("scoped package manifest must be written");
    std::fs::write(
        nested_package.join("package.json"),
        r#"{"name":"child-package","version":"2.0.0","scripts":{"postinstall":"node setup.js"}}"#,
    )
    .expect("nested package manifest must be written");

    let scripted = super::scan_scripted_packages(&node_modules)
        .expect("complete package tree must be scanned");

    assert!(scripted.contains(&("@scope/package".to_string(), "1.0.0".to_string())));
    assert!(scripted.contains(&("child-package".to_string(), "2.0.0".to_string())));
}
