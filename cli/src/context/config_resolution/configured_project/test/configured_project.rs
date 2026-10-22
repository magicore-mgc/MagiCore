//! Verify config-first identity adoption and read-only fallback behavior.
//! Kiểm tra nhận identity từ config và fallback khi thư mục chỉ đọc.

use crate::context::config_resolution::resolve_config;

#[test]
fn loading_legacy_config_claims_the_existing_core_identity() {
    let project = tempfile::tempdir().expect("temporary project directory must be created");
    std::fs::write(
        project.path().join("mgc.toml"),
        "name = \"legacy-web\"\nversion = \"0.1.0\"\necosystem = \"web\"\n",
    )
    .expect("legacy project config must be written");
    let root = project.path().to_path_buf();
    let (_, config) = resolve_config(&root, Some(&root), None)
        .expect("existing project config should load and claim its recorded core");

    assert_eq!(config.ecosystem, "web");
    assert_eq!(
        mgc_config::project::ProjectConfig::read_core_marker(&root)
            .expect("claimed marker must be readable")
            .as_deref(),
        Some("web"),
        "loading an existing config without a marker must persist its core owner"
    );
}

#[test]
fn matching_core_override_also_claims_legacy_config_identity() {
    let project = tempfile::tempdir().expect("temporary project directory must be created");
    std::fs::write(
        project.path().join("mgc.toml"),
        "name = \"legacy-web-override\"\nversion = \"0.1.0\"\necosystem = \"web\"\n",
    )
    .expect("legacy project config must be written");
    let root = project.path().to_path_buf();
    let (_, config) = resolve_config(&root, Some(&root), Some("web"))
        .expect("matching core override should preserve and claim the configured core");

    assert_eq!(config.ecosystem, "web");
    assert_eq!(
        mgc_config::project::ProjectConfig::read_core_marker(&root)
            .expect("claimed marker must be readable")
            .as_deref(),
        Some("web"),
        "a matching explicit override must not bypass identity-marker creation"
    );
}

#[cfg(unix)]
#[test]
fn read_only_legacy_project_still_loads_from_its_config_identity() {
    let project = tempfile::tempdir().expect("temporary project directory must be created");
    std::fs::write(
        project.path().join("mgc.toml"),
        "name = \"readonly-web\"\nversion = \"0.1.0\"\necosystem = \"web\"\n",
    )
    .expect("legacy project config must be written");
    let original_permissions = std::fs::metadata(project.path())
        .expect("project directory metadata must be readable")
        .permissions();
    let mut read_only_permissions = original_permissions.clone();
    read_only_permissions.set_readonly(true);
    std::fs::set_permissions(project.path(), read_only_permissions)
        .expect("project directory must become read-only");

    let root = project.path().to_path_buf();
    let result = resolve_config(&root, Some(&root), None);
    std::fs::set_permissions(project.path(), original_permissions)
        .expect("project directory must be writable for cleanup");

    let (_, config) = result.expect("read-only project config remains a valid identity source");
    assert_eq!(config.ecosystem, "web");
    assert!(
        mgc_config::project::ProjectConfig::read_core_marker(&root)
            .expect("missing optional marker must be readable")
            .is_none(),
        "read-only project must not fabricate a persisted marker"
    );
}
