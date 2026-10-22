use super::*;

/// Axum 0.8 capture syntax uses `{param}`; guard against route regressions.
/// (Axum 0.8 dùng `{param}` để bắt tham số; test ngăn route âm thầm trả 404.)
#[tokio::test]
async fn param_route_minimal() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::{Router, routing::get};
    use tower::ServiceExt;

    let app = Router::new().route("/a/{x}", get(|| async {}));
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/a/hello")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

/// Route matching: a matched route returns a handler-level status
/// (method/body/auth error or handler 404), never the router-level 404
/// of an unmatched path. Route syntax `:param` per matchit 0.7.
#[tokio::test]
async fn npm_routes_match() {
    use axum::Router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    let tmp = tempfile::tempdir().unwrap();
    let store = std::sync::Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    let auth = std::sync::Arc::new(AuthService::new(Some("adm".to_string()), store.clone()));
    let app = Router::new()
        .merge(mgc_registry_server::npm::routes())
        .with_state((store, auth));

    let cases = [
        // matched, handler rejects body (no Content-Type) -> 415
        ("/-/user/bob", "PUT", StatusCode::UNSUPPORTED_MEDIA_TYPE),
        // matched, package missing -> handler 404
        ("/-/package/x/dist-tags", "GET", StatusCode::NOT_FOUND),
        // matched, body rejected -> 415
        (
            "/-/package/x/dist-tags/latest",
            "PUT",
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
        ),
        // matched, no auth -> 401
        ("/-/whoami", "GET", StatusCode::UNAUTHORIZED),
        // matched, works without data
        ("/-/v1/search?q=x", "GET", StatusCode::OK),
        // matched, package missing -> handler 404
        ("/npm/foo", "GET", StatusCode::NOT_FOUND),
    ];
    for (path, method, expected) in cases {
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(path)
                    .method(method)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            resp.status(),
            expected,
            "{} {} -> {} (expected {})",
            method,
            path,
            resp.status(),
            expected
        );
    }
}

#[tokio::test]
async fn npm_delete_version_and_oci_tags_catalog() {
    use axum::Router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use sha2::Digest as _;
    use tower::ServiceExt;

    let tmp = tempfile::tempdir().unwrap();
    let store = std::sync::Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    let auth = std::sync::Arc::new(AuthService::new(Some("adm".to_string()), store.clone()));
    let app = Router::new()
        .merge(mgc_registry_server::npm::routes())
        .merge(mgc_registry_server::oci::routes())
        .with_state((store.clone(), auth));

    // npm: publish 1 version
    let pkg = serde_json::json!({
        "name": "demo-pkg",
        "maintainers": [],
        "versions": {
            "1.0.0": {
                "name": "demo-pkg",
                "version": "1.0.0",
                "_id": "demo-pkg@1.0.0",
                "_rev": "1",
                "dist": {"tarball": "http://x/demo-pkg-1.0.0.tgz", "shasum": "", "integrity": ""}
            }
        },
        "dist-tags": {"latest": "1.0.0"},
        "time": {"created":"","modified":""},
        "private": true
    });
    let put = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/npm/demo-pkg")
                .method("PUT")
                .header("content-type", "application/json")
                .header("authorization", "Bearer adm")
                .body(Body::from(pkg.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(put.status(), StatusCode::OK, "publish: {}", put.status());

    // DELETE version → package hết version → package bị xóa luôn
    let del = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/npm/demo-pkg/-/demo-pkg-1.0.0.tgz")
                .method("DELETE")
                .header("authorization", "Bearer adm")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        del.status(),
        StatusCode::NO_CONTENT,
        "delete version: {}",
        del.status()
    );

    let get = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/npm/demo-pkg")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        get.status(),
        StatusCode::NOT_FOUND,
        "package gone after delete"
    );

    // OCI: put manifest → tags/list + catalog thấy repo
    let config = b"test OCI config";
    let digest = format!("sha256:{}", hex::encode(sha2::Sha256::digest(config)));
    store.put_oci_blob("ai/m1", &digest, config).await.unwrap();
    let manifest = serde_json::json!({
        "schemaVersion": 2,
        "mediaType": "application/vnd.oci.image.manifest.v1+json",
        "config": {"mediaType": "application/vnd.oci.image.config.v1+json", "digest": digest, "size": config.len()},
        "layers": []
    })
    .to_string();
    let mput = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v2/ai/m1/manifests/1.0.0")
                .method("PUT")
                .header("content-type", "application/vnd.oci.image.manifest.v1+json")
                .header("authorization", "Bearer adm")
                .body(Body::from(manifest))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        mput.status(),
        StatusCode::CREATED,
        "manifest put: {}",
        mput.status()
    );

    let tags = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v2/ai/m1/tags/list")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let tags_body = axum::body::to_bytes(tags.into_body(), 4096).await.unwrap();
    let tags_json: serde_json::Value = serde_json::from_slice(&tags_body).unwrap();
    assert_eq!(
        tags_json["tags"],
        serde_json::json!(["1.0.0"]),
        "tags: {}",
        tags_json
    );

    let cat = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v2/_catalog")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let cat_body = axum::body::to_bytes(cat.into_body(), 4096).await.unwrap();
    let cat_json: serde_json::Value = serde_json::from_slice(&cat_body).unwrap();
    assert_eq!(
        cat_json["repositories"],
        serde_json::json!(["ai/m1"]),
        "catalog: {}",
        cat_json
    );
}
