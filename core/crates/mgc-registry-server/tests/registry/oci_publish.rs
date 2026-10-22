use super::*;

#[tokio::test]
async fn oci_manifest_requires_schema_and_all_verified_blobs_before_publish() {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use mgc_registry_server::{auth::AuthService, oci};
    use sha2::{Digest, Sha256};
    use std::sync::Arc;
    use tower::ServiceExt;

    let temp_dir = tempfile::tempdir().unwrap();
    let store = Arc::new(RegistryStore::new(temp_dir.path()).await.unwrap());
    let auth = Arc::new(AuthService::new(Some("admin-token".into()), store.clone()));
    let app = oci::routes().with_state((store.clone(), auth));

    let invalid = app
        .clone()
        .oneshot(
            Request::put("/v2/ai/models/weights/manifests/v1")
                .header("authorization", "Bearer admin-token")
                .header("content-type", "application/vnd.oci.image.manifest.v1+json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(invalid.status(), StatusCode::BAD_REQUEST);

    let config = b"config bytes";
    let layer = b"artifact bytes";
    let config_digest = format!("sha256:{}", hex::encode(Sha256::digest(config)));
    let layer_digest = format!("sha256:{}", hex::encode(Sha256::digest(layer)));
    let manifest = serde_json::json!({
        "schemaVersion": 2,
        "mediaType": "application/vnd.oci.image.manifest.v1+json",
        "config": {"mediaType": "application/vnd.oci.empty.v1+json", "digest": config_digest, "size": config.len()},
        "layers": [{"mediaType": "application/vnd.oci.image.layer.v1.tar", "digest": layer_digest, "size": layer.len()}]
    });
    let missing_blobs = app
        .clone()
        .oneshot(
            Request::put("/v2/ai/models/weights/manifests/v1")
                .header("authorization", "Bearer admin-token")
                .header("content-type", "application/vnd.oci.image.manifest.v1+json")
                .body(Body::from(manifest.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(missing_blobs.status(), StatusCode::BAD_REQUEST);
    assert!(
        store
            .get_oci_manifest("ai/models/weights", "v1")
            .await
            .unwrap()
            .is_none()
    );

    store
        .put_oci_blob("ai/models/weights", &config_digest, config)
        .await
        .unwrap();
    store
        .put_oci_blob("ai/models/weights", &layer_digest, layer)
        .await
        .unwrap();
    let invalid_reference = app
        .clone()
        .oneshot(
            Request::put("/v2/ai/models/weights/manifests/bad%0Aref")
                .header("authorization", "Bearer admin-token")
                .header("content-type", "application/vnd.oci.image.manifest.v1+json")
                .body(Body::from(manifest.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(invalid_reference.status(), StatusCode::BAD_REQUEST);
    assert!(
        store
            .get_oci_manifest("ai/models/weights", "bad\nref")
            .await
            .unwrap()
            .is_none()
    );
    let wrong_digest_reference = format!("sha256:{}", "0".repeat(64));
    let mismatched_digest = app
        .clone()
        .oneshot(
            Request::put(format!(
                "/v2/ai/models/weights/manifests/{wrong_digest_reference}"
            ))
            .header("authorization", "Bearer admin-token")
            .header("content-type", "application/vnd.oci.image.manifest.v1+json")
            .body(Body::from(manifest.to_string()))
            .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(mismatched_digest.status(), StatusCode::BAD_REQUEST);
    assert!(
        store
            .get_oci_manifest("ai/models/weights", &wrong_digest_reference)
            .await
            .unwrap()
            .is_none()
    );
    let duplicate_layer = manifest["layers"][0].clone();
    let mut too_many_descriptors = manifest.clone();
    too_many_descriptors["layers"] = serde_json::json!(vec![duplicate_layer; 10_000]);
    let descriptor_budget = app
        .clone()
        .oneshot(
            Request::put("/v2/ai/models/weights/manifests/too-many-descriptors")
                .header("authorization", "Bearer admin-token")
                .header("content-type", "application/vnd.oci.image.manifest.v1+json")
                .body(Body::from(too_many_descriptors.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(descriptor_budget.status(), StatusCode::BAD_REQUEST);
    assert!(
        store
            .get_oci_manifest("ai/models/weights", "too-many-descriptors")
            .await
            .unwrap()
            .is_none()
    );
    let mut wrong_size = manifest.clone();
    wrong_size["layers"][0]["size"] = serde_json::json!(layer.len() + 1);
    let rejected_size = app
        .clone()
        .oneshot(
            Request::put("/v2/ai/models/weights/manifests/v1")
                .header("authorization", "Bearer admin-token")
                .header("content-type", "application/vnd.oci.image.manifest.v1+json")
                .body(Body::from(wrong_size.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(rejected_size.status(), StatusCode::BAD_REQUEST);
    let published = app
        .clone()
        .oneshot(
            Request::put("/v2/ai/models/weights/manifests/v1")
                .header("authorization", "Bearer admin-token")
                .header("content-type", "application/vnd.oci.image.manifest.v1+json")
                .body(Body::from(manifest.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(published.status(), StatusCode::CREATED);
    assert_eq!(
        published.headers().get("location").unwrap(),
        "/v2/ai/models/weights/manifests/v1"
    );

    let (_, child_digest) = store
        .get_oci_manifest_raw("ai/models/weights", "v1")
        .await
        .unwrap()
        .unwrap();
    let child_size = manifest.to_string().len();
    let index = serde_json::json!({
        "schemaVersion": 2,
        "mediaType": "application/vnd.oci.image.index.v1+json",
        "manifests": [{
            "mediaType": "application/vnd.oci.image.manifest.v1+json",
            "digest": child_digest.clone(),
            "size": child_size,
            "platform": {"architecture": "amd64", "os": "linux"}
        }]
    });
    let index_published = app
        .clone()
        .oneshot(
            Request::put("/v2/ai/models/weights/manifests/multi")
                .header("authorization", "Bearer admin-token")
                .header("content-type", "application/vnd.oci.image.index.v1+json")
                .body(Body::from(index.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(index_published.status(), StatusCode::CREATED);
    let index_get = app
        .clone()
        .oneshot(
            Request::get("/v2/ai/models/weights/manifests/multi")
                .header("authorization", "Bearer admin-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(index_get.status(), StatusCode::OK);
    assert_eq!(
        index_get.headers().get("content-type").unwrap(),
        "application/vnd.oci.image.index.v1+json"
    );

    let docker_list = serde_json::json!({
        "schemaVersion": 2,
        "mediaType": "application/vnd.docker.distribution.manifest.list.v2+json",
        "manifests": [{
            "mediaType": "application/vnd.oci.image.manifest.v1+json",
            "digest": child_digest,
            "size": child_size,
            "platform": {"architecture": "arm64", "os": "linux"}
        }]
    });
    let list_published = app
        .clone()
        .oneshot(
            Request::put("/v2/ai/models/weights/manifests/docker-list")
                .header("authorization", "Bearer admin-token")
                .header(
                    "content-type",
                    "application/vnd.docker.distribution.manifest.list.v2+json",
                )
                .body(Body::from(docker_list.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(list_published.status(), StatusCode::CREATED);
    let list_get = app
        .oneshot(
            Request::get("/v2/ai/models/weights/manifests/docker-list")
                .header("authorization", "Bearer admin-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(list_get.status(), StatusCode::OK);
    assert_eq!(
        list_get.headers().get("content-type").unwrap(),
        "application/vnd.docker.distribution.manifest.list.v2+json"
    );
}

#[tokio::test]
async fn oci_chunked_upload_streams_to_verified_blob_store() {
    use axum::{body::Body, http::Request};
    use mgc_registry_server::{auth::AuthService, oci};
    use sha2::{Digest, Sha256};
    use std::sync::Arc;
    use tower::ServiceExt;

    let temp_dir = tempfile::tempdir().unwrap();
    let store = Arc::new(RegistryStore::new(temp_dir.path()).await.unwrap());
    let auth = Arc::new(AuthService::new(Some("admin-token".into()), store.clone()));
    let app = oci::routes().with_state((store.clone(), auth));
    let repository = "ai/models/weights";
    let bytes = b"large layer sent through the chunked route";
    let digest = format!("sha256:{}", hex::encode(Sha256::digest(bytes)));

    let started = app
        .clone()
        .oneshot(
            Request::post(format!("/v2/{repository}/blobs/uploads/"))
                .header("authorization", "Bearer admin-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(started.status(), axum::http::StatusCode::ACCEPTED);
    let upload_location = started
        .headers()
        .get("location")
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    let split = bytes.len() / 2;
    let chunked = app
        .clone()
        .oneshot(
            Request::patch(&upload_location)
                .header("authorization", "Bearer admin-token")
                .header("content-range", format!("0-{}", split - 1))
                .body(Body::from(bytes[..split].to_vec()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(chunked.status(), axum::http::StatusCode::ACCEPTED);
    assert_eq!(
        chunked.headers().get("range").unwrap(),
        format!("0-{}", split - 1).as_str()
    );

    let wrong_final_offset = app
        .clone()
        .oneshot(
            Request::put(format!("{upload_location}?digest={digest}"))
                .header("authorization", "Bearer admin-token")
                .header("content-range", format!("0-{}", bytes.len() - split - 1))
                .body(Body::from(bytes[split..].to_vec()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        wrong_final_offset.status(),
        axum::http::StatusCode::RANGE_NOT_SATISFIABLE
    );

    let finished = app
        .oneshot(
            Request::put(format!("{upload_location}?digest={digest}"))
                .header("authorization", "Bearer admin-token")
                .header("content-range", format!("{split}-{}", bytes.len() - 1))
                .body(Body::from(bytes[split..].to_vec()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(finished.status(), axum::http::StatusCode::CREATED);
    assert_eq!(
        store.get_oci_blob(repository, &digest).await.unwrap(),
        Some(bytes.to_vec())
    );
    assert!(
        store
            .oci_upload_path(repository, upload_location.rsplit('/').next().unwrap())
            .await
            .unwrap()
            .is_none()
    );
}

#[tokio::test]
async fn oci_upload_rejects_invalid_ranges_without_appending() {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use mgc_registry_server::{auth::AuthService, oci};
    use std::sync::Arc;
    use tower::ServiceExt;

    let temp_dir = tempfile::tempdir().unwrap();
    let store = Arc::new(RegistryStore::new(temp_dir.path()).await.unwrap());
    let repository = "ai/models/weights";
    let auth = Arc::new(AuthService::new(Some("admin-token".into()), store.clone()));
    let app = oci::routes().with_state((store.clone(), auth));
    let started = app
        .clone()
        .oneshot(
            Request::post(format!("/v2/{repository}/blobs/uploads/"))
                .header("authorization", "Bearer admin-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(started.status(), StatusCode::ACCEPTED);
    let location = started.headers().get("location").unwrap().to_str().unwrap();
    let uuid = location.rsplit('/').next().unwrap().to_owned();
    assert!(uuid::Uuid::parse_str(&uuid).is_ok());
    let upload_path = store
        .oci_upload_path(repository, &uuid)
        .await
        .unwrap()
        .unwrap();
    let uri = format!("/v2/{repository}/blobs/uploads/{uuid}");

    for range in [
        None,
        Some("0-99"),
        Some("1-3"),
        Some("bytes 0-2/*"),
        Some("garbage"),
    ] {
        let mut request = Request::patch(&uri)
            .header("authorization", "Bearer admin-token")
            .body(Body::from("abc"))
            .unwrap();
        if let Some(range) = range {
            request
                .headers_mut()
                .insert("content-range", range.parse().unwrap());
        }
        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::RANGE_NOT_SATISFIABLE);
        assert!(!tokio::fs::try_exists(&upload_path).await.unwrap());
    }

    let accepted = app
        .clone()
        .oneshot(
            Request::patch(&uri)
                .header("authorization", "Bearer admin-token")
                .header("content-range", "0-2")
                .body(Body::from("abc"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(accepted.status(), StatusCode::ACCEPTED);
    assert_eq!(accepted.headers().get("range").unwrap(), "0-2");
    assert_eq!(accepted.headers().get("location").unwrap(), uri.as_str());
    assert_eq!(tokio::fs::metadata(&upload_path).await.unwrap().len(), 3);

    let request = |body: &'static str| {
        Request::patch(&uri)
            .header("authorization", "Bearer admin-token")
            .header("content-range", "3-5")
            .body(Body::from(body))
            .unwrap()
    };
    let (first, second) = tokio::join!(
        app.clone().oneshot(request("def")),
        app.oneshot(request("xyz"))
    );
    let statuses = [first.unwrap().status(), second.unwrap().status()];
    assert_eq!(
        statuses
            .iter()
            .filter(|status| **status == StatusCode::ACCEPTED)
            .count(),
        1
    );
    assert_eq!(
        statuses
            .iter()
            .filter(|status| **status == StatusCode::RANGE_NOT_SATISFIABLE)
            .count(),
        1
    );
    let final_bytes = tokio::fs::read(&upload_path).await.unwrap();
    assert_eq!(final_bytes.len(), 6);
    assert!(final_bytes == b"abcdef" || final_bytes == b"abcxyz");
    assert!(!store.finish_oci_upload(repository, &uuid, 3).await.unwrap());
    assert!(store.finish_oci_upload(repository, &uuid, 6).await.unwrap());
    assert!(!tokio::fs::try_exists(&upload_path).await.unwrap());
}
