// Web command behavior tests — test hành vi command Web.
// Original setups and assertions are retained.

#[test]
fn v3_web_lockfile_is_written_with_current_schema_version() {
    // Deliberate v3 bump (Phase 1): web writes target the v3 schema.
    // Nâng lên v3 có chủ đích (Phase 1): web ghi ra nhắm schema v3.
    let lock = mgc_lockfile::Lockfile::new();
    let encoded = mgc_lockfile::serialization::to_toml(&lock).unwrap();
    assert!(encoded.contains("version = \"3\""));
    let decoded: mgc_lockfile::Lockfile = mgc_lockfile::serialization::from_toml(&encoded).unwrap();
    assert_eq!(decoded.version, "3");
}

#[test]
fn web_framework_aliases_resolve_to_supported_defaults() {
    for (alias, canonical) in [
        ("ts", "vanilla"),
        ("frontend", "vanilla"),
        ("custom", "vanilla"),
        ("node", "express"),
        ("backend", "express"),
        ("fullstack", "remix"),
        ("go", "gin"),
        ("java", "spring-boot"),
        ("php", "laravel"),
        ("python", "fastapi"),
        ("rust", "axum"),
        ("dotnet", "dotnet-minimal"),
        ("monorepo", "react-vite"),
    ] {
        assert_eq!(
            super::parse_framework_request(alias).normalized,
            canonical,
            "alias {alias}"
        );
    }
}

#[test]
fn web_framework_request_keeps_an_explicit_older_version() {
    let spec = crate::scaffold::spec::parse_scaffold_spec(
        crate::scaffold::spec::CoreKind::Web,
        "nextjs@15.2.0",
    )
    .expect("older framework version should parse as a scaffold ref");
    let request = super::framework_request_from_spec("nextjs", &spec.requested_ref);
    assert_eq!(request.normalized, "nextjs");
    assert_eq!(request.version.as_deref(), Some("15.2.0"));
    assert_eq!(request.dist_tag, None);
}

#[tokio::test]
async fn create_framework_version_reaches_the_primary_manifest_selector() {
    let spec = crate::scaffold::spec::parse_scaffold_spec(
        crate::scaffold::spec::CoreKind::Web,
        "nextjs@15.2.0",
    )
    .expect("older framework version should parse as a scaffold ref");
    let request = super::framework_request_from_spec("nextjs", &spec.requested_ref);
    let selected = super::resolve_primary_version(&request, "next", None)
        .await
        .expect("explicit framework version should not require registry access");

    assert_eq!(selected, "15.2.0");
}

#[test]
fn framework_dist_tag_is_preserved_for_registry_resolution() {
    let spec = crate::scaffold::spec::parse_scaffold_spec(
        crate::scaffold::spec::CoreKind::Web,
        "nextjs@beta",
    )
    .expect("dist tag should parse as a scaffold ref");
    let request = super::framework_request_from_spec("nextjs", &spec.requested_ref);

    assert_eq!(request.version, None);
    assert_eq!(request.dist_tag.as_deref(), Some("beta"));
}
