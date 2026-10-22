// Manifest path and size safety tests.
// Kiểm thử an toàn path và kích thước manifest.

use crate::scaffold::processor::MAX_TEMPLATE_MANIFEST_BYTES;

#[test]
fn manifest_paths_reject_cross_platform_escape_syntax() {
    let root = tempfile::tempdir().unwrap();
    for path in [
        "",
        "../outside",
        "a/../../outside",
        "a/.. /outside",
        "a/.. ./outside",
        "directory./file",
        "/absolute",
        "C:/outside",
        "a\\outside",
        "./file",
        "a//file",
        "a/",
        "a\nfile",
    ] {
        assert!(
            checked_template_path(root.path(), path).is_err(),
            "{path:?}"
        );
    }
    for path in [
        "src/main.rs",
        ".gitignore",
        "@scope/package/index.js",
        "file with spaces",
    ] {
        assert_eq!(
            checked_template_path(root.path(), path).unwrap(),
            root.path().join(path)
        );
    }
}
#[cfg(unix)]
#[test]
fn manifest_paths_reject_source_and_destination_links() {
    let root = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    std::fs::write(outside.path().join("secret"), "preserve").unwrap();
    std::os::unix::fs::symlink(outside.path(), root.path().join("sources")).unwrap();
    std::os::unix::fs::symlink(outside.path().join("secret"), root.path().join("output")).unwrap();
    assert!(checked_template_path(root.path(), "sources/secret").is_err());
    assert!(checked_template_path(root.path(), "output").is_err());
    assert_eq!(
        std::fs::read_to_string(outside.path().join("secret")).unwrap(),
        "preserve"
    );
}
#[test]
fn materialization_rejects_manifest_escape_before_writing_any_file_for_four_cores() {
    for core in ["web", "ai", "app", "lib"] {
        let root = tempfile::tempdir().unwrap();
        let layer_dir = root.path().join("layer");
        let target = root.path().join("project");
        std::fs::create_dir_all(layer_dir.join("sources")).unwrap();
        std::fs::create_dir(&target).unwrap();
        std::fs::write(layer_dir.join("sources/main.txt"), "untrusted").unwrap();
        for (source, destination) in [("main.txt", "../escaped"), ("../../secret", "output.txt")] {
            let manifest = format!(
                "[[files]]\nsource = 'main.txt'\ntarget = 'first.txt'\n[[files]]\nsource = '{source}'\ntarget = '{destination}'\n"
            );
            std::fs::write(layer_dir.join("template.toml"), manifest).unwrap();
            let layer = TemplateRoot::disk(layer_dir.clone());
            let config = ScaffoldConfig {
                core: core.to_string(),
                sub_type: "frontend".to_string(),
                frameworks: vec!["vanilla".to_string()],
                project_name: "probe".to_string(),
                features: vec![],
                template_dir: layer_dir.clone(),
            };
            let result = if core == "web" {
                let context = WebTemplateContext::new(&config, std::slice::from_ref(&layer));
                Scaffolder::materialize_template_layer(&target, &layer, &context)
            } else {
                let context = CoreTemplateContext::try_new(&config, "probe", "fixture").unwrap();
                Scaffolder::materialize_core_template(&target, &layer, &context)
            };
            assert!(result.is_err(), "{core}: {source} -> {destination}");
            assert!(!target.join("first.txt").exists());
            assert!(!root.path().join("escaped").exists());
        }
    }
}
#[test]
fn oversized_manifest_is_rejected_before_schema_parsing() {
    let root = tempfile::tempdir().unwrap();
    let file = std::fs::File::create(root.path().join("template.toml")).unwrap();
    file.set_len(MAX_TEMPLATE_MANIFEST_BYTES + 1).unwrap();
    let error = validate_template_manifest(root.path()).unwrap_err();
    assert!(error.to_string().contains("byte limit"));
}
