//! Registry/session policy tests.
//! Kiểm thử policy registry và phiên làm việc.
use super::*;

#[test]
fn publish_args_read_registry_from_mgc_registry_env() {
    use clap::CommandFactory as _;

    let command = crate::Cli::command();
    let publish = command
        .get_subcommands()
        .find(|subcommand| subcommand.get_name() == "publish")
        .expect("publish subcommand");
    let registry = publish
        .get_arguments()
        .find(|argument| argument.get_id() == "registry")
        .expect("publish registry argument");
    assert_eq!(
        registry.get_env().and_then(|value| value.to_str()),
        Some("MGC_REGISTRY")
    );
}

#[test]
fn trusted_oci_repository_uses_canonical_core_namespace() {
    assert_eq!(
        effective_trusted_oci_repository(Some("ai"), "models/weights").unwrap(),
        "ai/models/weights"
    );
    assert_eq!(
        effective_trusted_oci_repository(Some("clo"), "artifacts/app").unwrap(),
        "cloud/artifacts/app"
    );
    assert_eq!(
        effective_trusted_oci_repository(None, "acme/legacy-image").unwrap(),
        "acme/legacy-image"
    );
    assert!(effective_trusted_oci_repository(Some("ai"), "../escape").is_err());
    for core in [
        "web", "ai", "app", "lib", "game", "iot", "cloud", "cicd", "hardware",
    ] {
        assert_eq!(
            effective_trusted_oci_repository(Some(core), "pkg/artifact").unwrap(),
            format!("{core}/pkg/artifact")
        );
    }
}

#[test]
fn trusted_token_refresh_margin_and_docker_push_deadline_are_bounded() {
    let now = tokio::time::Instant::now();
    assert!(super::trusted_token_expires_within(
        now,
        now + std::time::Duration::from_secs(59),
        std::time::Duration::from_secs(60)
    ));
    assert!(!super::trusted_token_expires_within(
        now,
        now + std::time::Duration::from_secs(61),
        std::time::Duration::from_secs(60)
    ));
    assert_eq!(
        super::docker_push_timeout(std::time::Duration::from_secs(900)).unwrap(),
        std::time::Duration::from_secs(895)
    );
    assert_eq!(
        super::oci_request_timeout(std::time::Duration::from_secs(900)).unwrap(),
        std::time::Duration::from_secs(895)
    );
    assert!(super::docker_push_timeout(std::time::Duration::from_secs(5)).is_err());
    assert!(super::oci_request_timeout(std::time::Duration::from_secs(5)).is_err());
    assert_eq!(
        super::trusted_npm_request_timeout(std::time::Duration::from_secs(900)).unwrap(),
        std::time::Duration::from_secs(600)
    );
    assert_eq!(
        super::trusted_npm_request_timeout(std::time::Duration::from_secs(604)).unwrap(),
        std::time::Duration::from_secs(599)
    );
    assert!(super::trusted_npm_request_timeout(std::time::Duration::from_secs(5)).is_err());
}

#[test]
fn trusted_token_deadline_counts_exchange_network_time_against_ttl() {
    let exchange_started = tokio::time::Instant::now();
    let response_received = exchange_started + std::time::Duration::from_secs(14);
    let deadline =
        super::trusted_token_deadline(exchange_started, std::time::Duration::from_secs(900));

    assert_eq!(
        deadline,
        exchange_started + std::time::Duration::from_secs(900)
    );
    assert_eq!(
        deadline.saturating_duration_since(response_received),
        std::time::Duration::from_secs(886)
    );
}

#[test]
fn oci_upload_location_rejects_other_origins_and_unsafe_sessions() {
    let registry = url::Url::parse("https://registry.example").unwrap();
    assert!(
        super::safe_oci_upload_location(
            &registry,
            "ai/models/weights",
            "/v2/ai/models/weights/blobs/uploads/session-123?state=opaque"
        )
        .is_ok()
    );
    for location in [
        "https://attacker.example/v2/ai/models/weights/blobs/uploads/session",
        "/v2/other/package/blobs/uploads/session",
        "/v2/ai/models/weights/blobs/uploads/%2e%2e",
        "/v2/ai/models/weights/blobs/uploads/session/extra",
    ] {
        assert!(super::safe_oci_upload_location(&registry, "ai/models/weights", location).is_err());
    }
}

#[test]
fn trusted_oci_artifact_manifest_records_core_package_and_layer_digest() {
    let manifest = trusted_oci_artifact_manifest(
        "ai",
        "ai/models/weights",
        "1.0.0",
        &[(
            "weights.bin".to_owned(),
            format!("sha256:{}", "ab".repeat(32)),
            42,
        )],
    )
    .unwrap();
    let manifest: serde_json::Value = serde_json::from_slice(&manifest).unwrap();
    assert_eq!(manifest["schemaVersion"], 2);
    assert_eq!(
        manifest["mediaType"],
        "application/vnd.oci.image.manifest.v1+json"
    );
    assert_eq!(
        manifest["artifactType"],
        "application/vnd.magicore.artifact.v1"
    );
    assert_eq!(manifest["annotations"]["io.magicore.core"], "ai");
    assert_eq!(
        manifest["annotations"]["io.magicore.package"],
        "ai/models/weights"
    );
    assert_eq!(manifest["annotations"]["io.magicore.version"], "1.0.0");
    assert_eq!(
        manifest["layers"][0]["digest"],
        format!("sha256:{}", "ab".repeat(32))
    );
    assert_eq!(manifest["layers"][0]["size"], 42);
    assert_eq!(
        manifest["layers"][0]["annotations"]["org.opencontainers.image.title"],
        "weights.bin"
    );
    assert!(trusted_oci_artifact_manifest("web", "ai/models/weights", "1.0.0", &[]).is_err());
}
