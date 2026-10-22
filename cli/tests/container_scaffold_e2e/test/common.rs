//! Shared assertions for the container scaffold integration E2E.
//! Assertion dùng chung cho integration E2E scaffold container.

use std::path::Path;

use serde::Deserialize;

pub(super) fn assert_valid_k8s_manifest(manifest: &str, probe_path: &str) {
    let documents = serde_yaml::Deserializer::from_str(manifest)
        .map(serde_yaml::Value::deserialize)
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|error| panic!("invalid generated Kubernetes YAML: {error}"));
    assert_eq!(
        documents.len(),
        2,
        "manifest must contain Deployment and Service"
    );

    let deployment = &documents[0];
    assert_eq!(deployment["apiVersion"], "apps/v1");
    assert_eq!(deployment["kind"], "Deployment");
    assert_eq!(
        deployment["spec"]["template"]["spec"]["containers"][0]["startupProbe"]["httpGet"]["path"],
        probe_path
    );

    let service = &documents[1];
    assert_eq!(service["apiVersion"], "v1");
    assert_eq!(service["kind"], "Service");
    assert_eq!(service["spec"]["ports"][0]["targetPort"], "http");
}

pub(super) fn assert_project_roots_owned_before_switching_user(dockerfile: &str) {
    let ownership = dockerfile
        .find("RUN chown 65532:65532 /app /mgc-data")
        .expect("project and shared data roots must be writable by the runtime user");
    let user = dockerfile
        .rfind("USER 65532:65532")
        .expect("container must drop root privileges");
    assert!(
        ownership < user,
        "root ownership must be set before dropping privileges"
    );
}

pub(super) fn assert_mgc_tls_roots_are_available(dockerfile: &str) {
    assert!(
        dockerfile.contains(
            "COPY --from=mgc-cli /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt"
        ),
        "runtime image must carry the CA bundle used by MGC"
    );
    assert!(
        dockerfile.contains("SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt"),
        "MGC must use the copied CA bundle for HTTPS registry access"
    );
}

pub(super) fn assert_container_readme(project: &Path) {
    let readme = std::fs::read_to_string(project.join("README.md"))
        .expect("container-enabled project must document its run command");
    assert!(readme.contains("docker compose up --build"));
    assert!(readme.contains("copies that same lock"));
    assert!(readme.contains("mgc install --frozen"));
}

pub(super) fn assert_compose_configuration(project: &Path) {
    let compose_path = project.join("docker-compose.yml");
    let compose_text =
        std::fs::read_to_string(&compose_path).expect("--docker must create Compose configuration");
    let compose: serde_yaml::Value = serde_yaml::from_str(&compose_text).unwrap_or_else(|error| {
        panic!(
            "invalid Compose YAML at {}: {error}",
            compose_path.display()
        )
    });

    let service = compose
        .get("services")
        .and_then(|services| services.get("app"))
        .expect("Compose must define services.app");
    let service_volumes = service
        .get("volumes")
        .and_then(serde_yaml::Value::as_sequence)
        .expect("Compose app must mount the MGC shared data volume");
    assert!(
        service_volumes
            .iter()
            .any(|volume| volume.as_str() == Some("mgc-data:/mgc-data"))
    );

    let named_volume = compose
        .get("volumes")
        .and_then(|volumes| volumes.get("mgc-data"))
        .expect("Compose must declare the MGC named data volume");
    assert_eq!(
        named_volume.get("name").and_then(serde_yaml::Value::as_str),
        Some("${MGC_DATA_VOLUME:-magicore-system-data}")
    );
}
