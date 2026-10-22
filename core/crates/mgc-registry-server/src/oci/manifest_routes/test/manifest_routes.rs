use super::super::*;
use mgc_oidc::{OidcClaims, claims::Audience};

#[tokio::test]
async fn digest_reference_mismatch_precedes_trusted_signing_and_persistence() {
    let tmp = tempfile::tempdir().unwrap();
    let store = RegistryStore::new(tmp.path()).await.unwrap();
    let repository = "ai/models/weights";
    let config = b"config";
    let config_digest = format!("sha256:{}", hex::encode(Sha256::digest(config)));
    store
        .put_oci_blob(repository, &config_digest, config)
        .await
        .unwrap();
    let body = serde_json::to_vec(&serde_json::json!({
        "schemaVersion": 2,
        "mediaType": "application/vnd.oci.image.manifest.v1+json",
        "config": {
            "mediaType": "application/vnd.oci.empty.v1+json",
            "digest": config_digest,
            "size": config.len()
        },
        "layers": []
    }))
    .unwrap();
    let wrong_reference = format!("sha256:{}", "0".repeat(64));
    let identity = OidcClaims {
        iss: "https://token.actions.githubusercontent.com".into(),
        sub: "repo:acme/widgets:ref:refs/heads/main".into(),
        aud: Audience::Single("https://registry.example".into()),
        exp: 1_900_000_000,
        iat: 1_800_000_000,
        repository: Some("acme/widgets".into()),
        job_workflow_ref: None,
        workflow_ref: None,
        event_name: Some("push".into()),
    };
    let key = mgc_crypto::KeyPair::generate().unwrap();

    let result = manifest_put(
        &store,
        repository,
        &wrong_reference,
        Bytes::from(body),
        Some((identity, 1, "not-a-jwt".into())),
        Some(&key),
    )
    .await;

    assert!(matches!(result, Err(StatusCode::BAD_REQUEST)));
    assert!(
        store
            .get_oci_manifest(repository, &wrong_reference)
            .await
            .unwrap()
            .is_none()
    );
    assert!(
        store
            .trusted_attestations("oci:ai/models/weights")
            .await
            .unwrap()
            .is_empty()
    );
}
