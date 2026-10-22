use super::*;

#[tokio::test]
async fn oci_routes_match() {
    use axum::Router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    let tmp = tempfile::tempdir().unwrap();
    let store = std::sync::Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    let auth = std::sync::Arc::new(AuthService::new(Some("adm".to_string()), store.clone()));
    let app = Router::new()
        .merge(mgc_registry_server::oci::routes())
        .with_state((store, auth));

    // HEAD blob on missing digest -> 404 from handler (route matched)
    let head = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v2/ai/mymodel/blobs/sha256:deadbeef")
                .method("HEAD")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        head.status(),
        StatusCode::NOT_FOUND,
        "blob HEAD: {}",
        head.status()
    );

    // POST uploads -> 202 with location — POST tạo session upload.
    let post = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v2/ai/mymodel/blobs/uploads/")
                .method("POST")
                .header("authorization", "Bearer adm")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        post.status(),
        StatusCode::ACCEPTED,
        "upload start: {}",
        post.status()
    );

    // tags list -> 200
    let tags = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v2/ai/mymodel/tags/list")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        tags.status(),
        StatusCode::OK,
        "tags/list: {}",
        tags.status()
    );

    // catalog -> 200
    let cat = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v2/_catalog")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(cat.status(), StatusCode::OK, "catalog: {}", cat.status());
}
