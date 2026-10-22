#![allow(clippy::unwrap_used)]

mod common;
const FRAMEWORK: &str = "gin";
include!("templates/backend.rs");

#[test]
fn default_gin_scaffold_is_native_and_pins_its_go_module() {
    let project = common::scaffold(FRAMEWORK, "native-gin");
    let manifest = std::fs::read_to_string(project.join("go.mod"))
        .expect("native Gin scaffold writes a Go module manifest");
    let source = std::fs::read_to_string(project.join("main.go"))
        .expect("native Gin scaffold writes its server entrypoint");

    assert!(manifest.contains("github.com/gin-gonic/gin v1.12.0"));
    assert!(source.contains("/api/health"));
}
