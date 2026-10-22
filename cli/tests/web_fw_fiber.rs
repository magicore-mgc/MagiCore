#![allow(clippy::unwrap_used)]

mod common;
const FRAMEWORK: &str = "fiber";
include!("templates/backend.rs");

#[test]
fn default_fiber_scaffold_is_native_and_pins_its_go_module() {
    let project = common::scaffold(FRAMEWORK, "native-fiber");
    let manifest = std::fs::read_to_string(project.join("go.mod"))
        .expect("native Fiber scaffold writes a Go module manifest");
    let source = std::fs::read_to_string(project.join("main.go"))
        .expect("native Fiber scaffold writes its server entrypoint");

    assert!(manifest.contains("github.com/gofiber/fiber/v3 v3.6.1"));
    assert!(source.contains("/api/health"));
}
