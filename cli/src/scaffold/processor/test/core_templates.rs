// Cross-core template materialization tests.
// Kiểm thử materialize template đa core.

#[test]
fn test_display_name_uses_last_path_segment() {
    let path = Path::new("/tmp/my-project");
    assert_eq!(Scaffolder::display_name(path), "my-project");
}
#[test]
fn test_lib_templates_materialize_all_languages() {
    if !template_layer_ready("lib/ts") {
        eprintln!("skipped: lib/ts template layer not available offline (registry-first)");
        return;
    }
    let root = tempfile::tempdir().unwrap();
    for (language, manifest, marker) in [
        ("ts", "package.json", "\"core\""),
        ("rust", "Cargo.toml", "core = \"lib\""),
        ("python", "pyproject.toml", "core = \"lib\""),
    ] {
        let project_dir = root.path().join(format!("demo-{language}"));
        let config = ScaffoldConfig {
            core: "lib".to_string(),
            sub_type: String::new(),
            frameworks: vec![language.to_string()],
            project_name: project_dir.to_string_lossy().to_string(),
            features: vec![],
            template_dir: PathBuf::new(),
            from_local: None,
            no_local: false,
        };

        let out = Scaffolder::scaffold(&config).unwrap();
        assert_eq!(out, project_dir);
        assert!(out.join(manifest).exists(), "{language} manifest");
        let mgc = std::fs::read_to_string(out.join("mgc.toml")).unwrap();
        assert!(
            mgc.contains("ecosystem = \"lib\""),
            "{language} mgc.toml ecosystem"
        );
        assert!(
            mgc.contains(&format!("language = \"{language}\"")),
            "{language} language"
        );
        let native = std::fs::read_to_string(out.join(manifest)).unwrap();
        assert!(native.contains(marker), "{language} marker");
        if language == "ts" {
            assert!(
                native.contains("\"typescript\": \"^5\""),
                "ts scaffold devDeps typescript"
            );
        }
    }
    let py_src = root
        .path()
        .join("demo-python")
        .join("src")
        .join("demo_python")
        .join("__init__.py");
    assert!(py_src.exists(), "python package source");
}
#[test]
fn test_game_templates_materialize_all_engines() {
    if !template_layer_ready("game/bevy") {
        eprintln!("skipped: game/bevy template layer not available offline (registry-first)");
        return;
    }
    let root = tempfile::tempdir().unwrap();
    for (framework, manifest) in [
        ("bevy", "Cargo.toml"),
        ("godot", "project.godot"),
        ("unity", "Packages/manifest.json"),
        ("unreal", "demo-unreal.uproject"),
    ] {
        let project_dir = root.path().join(format!("demo-{framework}"));
        let config = ScaffoldConfig {
            core: "game".to_string(),
            sub_type: String::new(),
            frameworks: vec![framework.to_string()],
            project_name: project_dir.to_string_lossy().to_string(),
            features: vec![],
            template_dir: PathBuf::new(),
            from_local: None,
            no_local: false,
        };

        let out = Scaffolder::scaffold(&config).unwrap();
        assert_eq!(out, project_dir);
        assert!(out.join(manifest).exists(), "{framework} manifest");
        let mgc = std::fs::read_to_string(out.join("mgc.toml")).unwrap();
        assert!(
            mgc.contains("ecosystem = \"game\""),
            "{framework} mgc.toml ecosystem"
        );
        assert!(
            mgc.contains(&format!("engine = \"{framework}\"")),
            "{framework} engine"
        );
    }
    let bevy_src = root.path().join("demo-bevy").join("src").join("main.rs");
    assert!(bevy_src.exists(), "bevy source");
}
#[test]
fn test_optimizer_template_materializes() {
    if !template_layer_ready("hardware/optimizer") {
        eprintln!(
            "skipped: hardware/optimizer template layer not available offline (registry-first)"
        );
        return;
    }
    let root = tempfile::tempdir().unwrap();
    let optimizer_dir = root.path().join("optimizer");
    let config = ScaffoldConfig {
        core: "hardware".to_string(),
        sub_type: String::new(),
        frameworks: vec!["optimizer".to_string()],
        project_name: optimizer_dir.to_string_lossy().to_string(),
        features: vec![],
        template_dir: PathBuf::new(),
        from_local: None,
        no_local: false,
    };

    let out = Scaffolder::scaffold(&config).unwrap();
    assert_eq!(out, optimizer_dir);
    assert!(out.join("Cargo.toml").exists(), "optimizer Cargo.toml");
    assert!(out.join("src").join("lib.rs").exists(), "optimizer lib.rs");
    assert!(out.join("build.rs").exists(), "optimizer build.rs");
    assert!(
        out.join("shaders").join("compute.wgsl").exists(),
        "optimizer shader"
    );
    let cargo = std::fs::read_to_string(out.join("Cargo.toml")).unwrap();
    assert!(
        cargo.contains("name = \"mgc-optimizer\""),
        "fixed package name"
    );
    assert!(
        cargo.contains("[workspace]"),
        "workspace opt-out for nested crates"
    );
    let lib = std::fs::read_to_string(out.join("src").join("lib.rs")).unwrap();
    assert!(lib.contains("mgc_optimizer_init"), "FFI init export");
    assert!(
        lib.contains("mgc_optimizer_optimize_mesh"),
        "FFI mesh export"
    );
}
