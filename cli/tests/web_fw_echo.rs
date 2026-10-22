#![allow(clippy::unwrap_used)]

mod common;
const FRAMEWORK: &str = "echo";
include!("templates/backend.rs");

#[test]
fn default_echo_scaffold_is_native_and_pins_its_go_module() {
    let project = common::scaffold(FRAMEWORK, "native-echo");
    let manifest = std::fs::read_to_string(project.join("go.mod"))
        .expect("native Echo scaffold writes a Go module manifest");
    let source = std::fs::read_to_string(project.join("main.go"))
        .expect("native Echo scaffold writes its server entrypoint");

    assert!(manifest.contains("github.com/labstack/echo/v5 v5.4.0"));
    assert!(source.contains("/api/health"));
}
