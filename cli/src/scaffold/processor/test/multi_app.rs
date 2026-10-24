// Multi-platform application scaffold test.
// Kiểm thử scaffold ứng dụng đa nền tảng.
//
// `multi` used to be synthesized by a compiled processor. MagiCore no longer
// embeds framework source, so without a registry layer or a reusable local
// project the scaffold must fail closed instead of writing platform trees.
// `multi` từng được processor biên dịch tự sinh. MagiCore không còn nhúng
// source framework, nên thiếu registry layer hoặc project local thì scaffold
// phải fail-closed thay vì ghi cây platform.

#[test]
fn test_multi_app_scaffold_without_layer_or_local_source_fails_closed() {
    let root = tempfile::tempdir().unwrap();
    let project_dir = root.path().join("demo-multi");
    let config = ScaffoldConfig {
        core: "app".to_string(),
        sub_type: String::new(),
        frameworks: vec!["multi".to_string()],
        project_name: project_dir.to_string_lossy().to_string(),
        features: vec![],
        template_dir: PathBuf::new(),
        from_local: None,
        no_local: true,
    };

    let error = Scaffolder::scaffold(&config).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("No scaffold source for core 'app' framework 'multi'"),
        "{error}"
    );
    assert!(
        !project_dir.exists(),
        "failed multi scaffold must not leave a partial project"
    );
}
