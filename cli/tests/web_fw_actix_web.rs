#![allow(clippy::unwrap_used)]

mod common;
const FRAMEWORK: &str = "actix-web";
include!("templates/backend.rs");

#[test]
fn default_actix_web_scaffold_has_a_native_health_server() {
    let dir = common::scaffold(FRAMEWORK, "test-actix-web-native-source");
    let manifest = std::fs::read_to_string(dir.join("Cargo.toml")).unwrap();
    let source = std::fs::read_to_string(dir.join("src/main.rs")).unwrap();
    assert!(manifest.contains("actix-web = \"4\""));
    assert!(source.contains("/api/health"));
    assert!(source.contains("HttpServer::new"));
}
