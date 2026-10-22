// Template-root and all-core baseline tests.
// Kiểm thử template root và baseline toàn core.

#[test]
fn test_disk_template_root_reads_manifest() {
    use crate::scaffold::template_root::TemplateRoot;
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("sources")).unwrap();
    std::fs::write(dir.path().join("template.toml"), "[files]\n").unwrap();
    let root = TemplateRoot::disk(dir.path().to_path_buf());
    let bytes = root.read("template.toml").unwrap();
    let text = String::from_utf8(bytes).unwrap();
    assert!(text.contains("files"), "manifest has files");
    assert!(root.exists("sources"), "sources dir visible");
}
#[test]
fn test_scaffold_writes_baseline_for_all_cores() {
    if !template_layer_ready("web/frontend/react-vite") {
        eprintln!(
            "skipped: web/frontend/react-vite template layer not available offline (registry-first)"
        );
        return;
    }
    let root = tempfile::tempdir().unwrap();
    let cases = [
        ("web", "react-vite", "package.json"),
        ("game", "bevy", "Cargo.toml"),
        ("ai", "python-agent", "pyproject.toml"),
        ("clo", "pulumi-aws", "Pulumi.yaml"),
        ("cicd", "github-actions", ".github/workflows/ci.yml"),
        ("iot", "esp32-rust", "Cargo.toml"),
        ("app", "flutter", "pubspec.yaml"),
        ("lib", "rust", "Cargo.toml"),
    ];

    for (core, framework, expected) in cases {
        let project_dir = root.path().join(format!("{core}-{framework}"));
        let config = ScaffoldConfig {
            core: core.to_string(),
            sub_type: String::new(),
            frameworks: vec![framework.to_string()],
            project_name: project_dir.to_string_lossy().to_string(),
            features: vec![],
            template_dir: PathBuf::new(),
        };

        let out = Scaffolder::scaffold(&config).unwrap();
        assert!(out.join(expected).exists(), "{} {}", core, expected);
        assert!(out.join("README.md").exists(), "{} README", core);
        if core == "web" {
            assert!(out.join("mgc.lock").exists(), "web mgc.lock");
            assert!(out.join("mgc.toml").exists(), "web mgc.toml");
        }
    }
}
