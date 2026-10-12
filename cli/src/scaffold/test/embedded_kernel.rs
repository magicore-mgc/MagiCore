use super::*;

#[test]
fn test_get_embedded_template_returns_files_for_react_axum_fastapi() {
    let react = get_embedded_template("web", "react").unwrap();
    assert!(react.iter().any(|f| f.path == "package.json"));
    assert!(react.iter().any(|f| f.path == "vite.config.ts"));

    let axum = get_embedded_template("web", "axum").unwrap();
    assert!(axum.iter().any(|f| f.path == "Cargo.toml"));
    assert!(axum.iter().any(|f| f.path == "src/main.rs"));

    let fastapi = get_embedded_template("web", "fastapi").unwrap();
    assert!(fastapi.iter().any(|f| f.path == "main.py"));
    assert!(fastapi.iter().any(|f| f.path == "pyproject.toml"));
}

#[test]
fn test_angular_dev_server_references_its_build_target() {
    let angular = get_embedded_template("web", "angular").unwrap();
    let angular_json = angular
        .iter()
        .find(|file| file.path == "angular.json")
        .expect("Angular template should include angular.json");
    let config: serde_json::Value =
        serde_json::from_str(angular_json.content).expect("Angular config should be valid JSON");

    assert_eq!(
        config["projects"]["{{PROJECT_NAME}}"]["architect"]["serve"]["options"]["buildTarget"],
        "{{PROJECT_NAME}}:build",
        "Angular dev-server must point at its generated build target"
    );
}

#[test]
fn test_remix_vite_template_uses_native_dev_launch_command() {
    let remix = get_embedded_template("web", "remix").unwrap();
    let package_json = remix
        .iter()
        .find(|file| file.path == "package.json")
        .expect("Remix template should include package.json");
    let package: serde_json::Value =
        serde_json::from_str(package_json.content).expect("Remix package.json should be valid");

    assert_eq!(package["scripts"]["dev"], "remix vite:dev --port 4315");
    assert_eq!(package["scripts"]["build"], "remix vite:build");
}

#[test]
fn test_materialize_embedded_performs_in_memory_replacement() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("my-fast-app");
    let files = get_embedded_template("web", "react").unwrap();

    let res = materialize_embedded(&root, "my-fast-app", &files);
    assert!(res.is_ok());

    let pkg_json = std::fs::read_to_string(root.join("package.json")).unwrap();
    assert!(pkg_json.contains(r#""name": "my-fast-app""#));

    let index_html = std::fs::read_to_string(root.join("index.html")).unwrap();
    assert!(index_html.contains("<title>my-fast-app</title>"));
}
