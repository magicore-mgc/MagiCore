use std::fs;
use tempfile;

use super::super::*;

#[test]
fn version_marker_alone_is_not_a_usable_layer() {
    let root = tempfile::tempdir().unwrap();
    fs::write(root.path().join(".mgc-version"), "1.0.0").unwrap();
    assert!(!template_layer_has_contract(root.path()).unwrap());
    fs::write(
        root.path().join("template.toml"),
        "[[files]]\nsource = 'main.txt'\ntarget = 'main.txt'\n",
    )
    .unwrap();
    assert!(!template_layer_has_contract(root.path()).unwrap());
    fs::create_dir(root.path().join("sources")).unwrap();
    assert!(template_layer_has_contract(root.path()).unwrap());
}

#[test]
fn syntactically_valid_toml_without_processor_schema_preserves_existing_cache() {
    let root = tempfile::tempdir().unwrap();
    let target = root.path().join("layer");
    fs::create_dir(&target).unwrap();
    fs::write(target.join("sentinel"), "preserve").unwrap();
    let bytes = archive(&[
        (
            "package/template.toml",
            b"[template]\nname = 'unusable'\n",
            tar::EntryType::Regular,
        ),
        (
            "package/sources/main.txt",
            b"content",
            tar::EntryType::Regular,
        ),
    ]);
    assert!(extract_tarball_into(&bytes, &target, Some("1.0.0")).is_err());
    assert_eq!(
        fs::read_to_string(target.join("sentinel")).unwrap(),
        "preserve"
    );
    assert!(!target.join(".mgc-version").exists());
    fs::write(
        target.join("template.toml"),
        "[template]\nname = 'unusable'\n",
    )
    .unwrap();
    fs::create_dir(target.join("sources")).unwrap();
    assert!(template_layer_has_contract(&target).is_err());
}
