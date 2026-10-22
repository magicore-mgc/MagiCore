// Test-only template helpers.
// Helper fixture chỉ dùng trong test.

/// Registry-first: template layer cần fetch/cache sẵn (~/.mgc/templates hoặc
/// MGC_TEMPLATES_DIR). Máy sạch offline → skip test materialize.
fn template_layer_ready(rel: &str) -> bool {
    let root = crate::scaffold::template_root::TemplateRoot::resolve(rel);
    root.exists("template.toml") && root.exists("sources")
}
fn online_web_template_fixture() -> tempfile::TempDir {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(
        root.path().join("package.json"),
        "{\"name\":\"web-fixture\",\"private\":true,\"version\":\"0.1.0\"}\n",
    )
    .unwrap();
    std::fs::write(
        root.path().join("index.html"),
        "<!doctype html><html></html>\n",
    )
    .unwrap();
    std::fs::create_dir_all(root.path().join("src")).unwrap();
    std::fs::write(root.path().join("src/main.ts"), "export {};\n").unwrap();
    root
}
