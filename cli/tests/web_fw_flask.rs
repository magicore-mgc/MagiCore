#![allow(clippy::unwrap_used)]

mod common;
const FRAMEWORK: &str = "flask";
include!("templates/backend.rs");

#[test]
fn default_flask_scaffold_is_native_and_contains_a_health_route() {
    let project = common::scaffold(FRAMEWORK, "native-flask");
    let manifest = std::fs::read_to_string(project.join("pyproject.toml"))
        .expect("native Flask scaffold writes a Python project manifest");
    let source = std::fs::read_to_string(project.join("main.py"))
        .expect("native Flask scaffold writes its server entrypoint");

    assert!(manifest.contains("flask>=3.0"));
    assert!(source.contains("@app.get(\"/api/health\")"));
    assert!(source.contains("app.run(host=\"0.0.0.0\""));
}
