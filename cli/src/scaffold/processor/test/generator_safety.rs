// Generator and generated-asset safety tests.
// Kiểm thử an toàn generator và asset sinh ra.

#[test]
fn template_layers_cannot_emit_cli_owned_container_assets() {
    for target in [
        "Dockerfile",
        ".dockerignore",
        "docker-compose.yml",
        "mgc-container-entrypoint",
        "k8s/deployment.yaml",
    ] {
        let file = TemplateFile {
            source: target.to_string(),
            target: target.to_string(),
            required_context: vec![],
            include_features: vec![],
            exclude_features: vec![],
        };
        assert!(file.is_cli_generated_container_asset(), "{target}");
    }

    let feature_owned = TemplateFile {
        source: "runtime.config".to_string(),
        target: "runtime.config".to_string(),
        required_context: vec![],
        include_features: vec!["docker".to_string()],
        exclude_features: vec![],
    };
    assert!(feature_owned.is_cli_generated_container_asset());

    let framework_source = TemplateFile {
        source: "src/server.ts".to_string(),
        target: "src/server.ts".to_string(),
        required_context: vec![],
        include_features: vec![],
        exclude_features: vec![],
    };
    assert!(!framework_source.is_cli_generated_container_asset());
}
#[test]
fn online_generator_copy_omits_package_cache_and_git_metadata() {
    let source = tempfile::tempdir().unwrap();
    let target = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(source.path().join("src")).unwrap();
    std::fs::create_dir_all(source.path().join("node_modules/unused")).unwrap();
    std::fs::create_dir_all(source.path().join(".git/objects")).unwrap();
    std::fs::write(source.path().join("src/main.ts"), "export {};\n").unwrap();
    std::fs::write(
        source.path().join("node_modules/unused/package.json"),
        "{}\n",
    )
    .unwrap();
    std::fs::write(source.path().join(".git/objects/data"), "private\n").unwrap();

    Scaffolder::copy_online_generated_template(source.path(), target.path()).unwrap();

    assert!(target.path().join("src/main.ts").is_file());
    assert!(!target.path().join("node_modules").exists());
    assert!(!target.path().join(".git").exists());
}
#[cfg(unix)]
#[test]
fn online_generator_copy_rejects_symlinks() {
    let source = tempfile::tempdir().unwrap();
    let target = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    std::fs::write(outside.path().join("secret"), "must not copy\n").unwrap();
    std::os::unix::fs::symlink(outside.path(), source.path().join("linked"))
        .expect("create symlink fixture");

    let error = Scaffolder::copy_online_generated_template(source.path(), target.path())
        .expect_err("generator symlinks must be rejected");

    assert!(error.to_string().contains("linked or special path"));
    assert!(!target.path().join("secret").exists());
}

#[test]
fn hybrid_nuxt_scaffold_ignores_storage_without_hiding_build_assets() {
    for framework in ["nuxt", "react-vite"] {
        let source = tempfile::tempdir().unwrap();
        let target = tempfile::tempdir().unwrap();
        let original = "custom/**\n**/.magicore/cache\n";
        std::fs::write(source.path().join(".nuxtignore"), original).unwrap();
        std::fs::write(source.path().join("package.json"), r#"{"name":"sample","private":true}"#).unwrap();
        let mut config = Scaffolder::infer_web_create_config(framework, "sample").unwrap();
        config.template_dir = source.path().to_path_buf();
        Scaffolder::write_web_files(target.path(), &config, "sample", framework).unwrap();
        let contents = std::fs::read_to_string(target.path().join(".nuxtignore")).unwrap();
        if framework == "nuxt" {
            assert!(contents.starts_with(original));
            assert_eq!(contents.lines().filter(|line| *line == "**/.magicore/cache").count(), 1);
            assert!(contents.contains("**/.magicore/store"));
            assert!(contents.contains("**/.magicore/exec.log"));
            assert!(!contents.lines().any(|line| matches!(line, "**/.magicore" | "**/.magicore/build")));
        } else {
            assert_eq!(contents, original);
        }
    }
}
