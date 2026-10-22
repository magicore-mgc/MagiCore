use super::super::*;

#[test]
fn cargo_projection_contains_root_transitive_edges_and_checksums() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[dependencies]\nfixture = \"1\"\n",
    )
    .unwrap();
    let fixture = locked_package("fixture", &["child"]);
    let child = locked_package("child", &[]);
    let packages = vec![&fixture, &child];

    let bytes =
        project_cargo_lock(project.path(), &packages, &["fixture@1.0.0".to_string()]).unwrap();
    let document: toml::Value = toml::from_str(std::str::from_utf8(&bytes).unwrap()).unwrap();
    let rows = document["package"].as_array().unwrap();
    assert_eq!(rows.len(), 3);
    let fixture_row = rows
        .iter()
        .find(|row| row["name"].as_str() == Some("fixture"))
        .unwrap();
    let fixture_checksum = "ab".repeat(32);
    assert_eq!(
        fixture_row["checksum"].as_str(),
        Some(fixture_checksum.as_str())
    );
    assert_eq!(fixture_row["dependencies"][0].as_str(), Some("child"));
    assert!(rows.iter().any(|row| row["name"].as_str() == Some("demo")));

    // V4 SRI and native hex must yield byte-identical Cargo projections.
    // Toolchain Cargo vẫn nhận checksum hex khi mgc.lock V4 lưu SRI base64.
    use base64::Engine as _;
    let mut sri_fixture = fixture.clone();
    let mut sri_child = child.clone();
    for package in [&mut sri_fixture, &mut sri_child] {
        package.integrity = format!(
            "sha256-{}",
            base64::engine::general_purpose::STANDARD.encode([0xab; 32])
        );
    }
    let sri_lock = Lockfile {
        packages: vec![sri_fixture, sri_child],
        ..Lockfile::new()
    };
    let sri_packages = rust_packages(&sri_lock).unwrap();
    let sri_bytes = project_cargo_lock(
        project.path(),
        &sri_packages,
        &["fixture@1.0.0".to_string()],
    )
    .unwrap();
    assert_eq!(
        sri_bytes, bytes,
        "digest encoding must not change Cargo.lock bytes"
    );
}

#[test]
fn cargo_projection_rejects_unsupported_tables_and_accepts_multiple_versions() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[dependencies]\nfixture = { path = \"../fixture\" }\n",
    )
    .unwrap();
    let fixture = locked_package("fixture", &[]);
    let error = project_cargo_lock(project.path(), &[&fixture], &["fixture@1.0.0".to_string()])
        .unwrap_err();
    assert!(error.to_string().contains("path/git/workspace"));

    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[dependencies]\nfixture = \"1\"\n",
    )
    .unwrap();
    let mut other = locked_package("fixture", &[]);
    other.version = "2.0.0".to_string();
    let lock = Lockfile {
        packages: vec![fixture.clone(), other],
        ..Lockfile::new()
    };
    assert_eq!(rust_packages(&lock).unwrap().len(), 2);
}

#[test]
fn cargo_projection_keeps_dependency_edges_between_same_named_versions() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[dependencies]\nfixture = \"1\"\n",
    )
    .unwrap();
    let mut fixture = locked_package("fixture", &["shared@0.6.2", "shared@0.8.0"]);
    fixture.version = "1.0.0".to_string();
    let mut shared_06 = locked_package("shared", &[]);
    shared_06.version = "0.6.2".to_string();
    let mut shared_08 = locked_package("shared", &[]);
    shared_08.version = "0.8.0".to_string();
    let packages = vec![&fixture, &shared_06, &shared_08];

    let bytes =
        project_cargo_lock(project.path(), &packages, &["fixture@1.0.0".to_string()]).unwrap();
    let document: toml::Value = toml::from_str(std::str::from_utf8(&bytes).unwrap()).unwrap();
    let fixture_row = document["package"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["name"].as_str() == Some("fixture"))
        .unwrap();
    let mut edges = fixture_row["dependencies"]
        .as_array()
        .unwrap()
        .iter()
        .map(|edge| edge.as_str().unwrap().to_string())
        .collect::<Vec<_>>();
    edges.sort();
    assert_eq!(edges, ["shared 0.6.2", "shared 0.8.0"]);
    assert_eq!(
        document["package"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|row| row["name"].as_str() == Some("shared"))
            .count(),
        2
    );
}

#[test]
fn cargo_projection_accepts_dependency_feature_options() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"esp_demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[dependencies]\nesp-hal = { version = \"1.2.2\", features = [\"esp32c3\"], default-features = false }\n",
    )
    .unwrap();
    let mut esp_hal = locked_package("esp-hal", &[]);
    esp_hal.version = "1.2.2".to_string();

    let bytes =
        project_cargo_lock(project.path(), &[&esp_hal], &["esp-hal@1.2.2".to_string()]).unwrap();
    let document: toml::Value = toml::from_str(std::str::from_utf8(&bytes).unwrap()).unwrap();
    assert!(document["package"].as_array().unwrap().iter().any(|row| {
        row["name"].as_str() == Some("esp-hal") && row["version"].as_str() == Some("1.2.2")
    }));
}
