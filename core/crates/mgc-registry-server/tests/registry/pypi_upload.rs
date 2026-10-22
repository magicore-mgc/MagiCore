use super::*;

#[tokio::test]
async fn pypi_upload_index_download() {
    use axum::Router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use sha2::{Digest, Sha256};
    use tower::ServiceExt;

    let tmp = tempfile::tempdir().unwrap();
    let store = std::sync::Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    let auth = std::sync::Arc::new(AuthService::new(Some("adm".to_string()), store.clone()));
    let app = Router::new()
        .merge(mgc_registry_server::pypi::routes())
        .with_state((store, auth));

    // Upload wheel content (ASCII — multipart test body là string literal)
    let wheel = b"dummy wheel content for test";
    let mut hasher = Sha256::new();
    hasher.update(wheel);
    let digest = format!("sha256:{}", hex::encode(hasher.finalize()));

    // multipart upload → twine format
    let boundary = "MGBTEST";
    let body = format!(
        "--{b}\r\nContent-Disposition: form-data; name=\":action\"\r\n\r\nfile_upload\r\n\
         --{b}\r\nContent-Disposition: form-data; name=\"name\"\r\n\r\ndemo-pkg\r\n\
         --{b}\r\nContent-Disposition: form-data; name=\"version\"\r\n\r\n1.0.0\r\n\
         --{b}\r\nContent-Disposition: form-data; name=\"sha256_digest\"\r\n\r\n{sha}\r\n\
         --{b}\r\nContent-Disposition: form-data; name=\"requires_python\"\r\n\r\n>=3.11\r\n\
         --{b}\r\nContent-Disposition: form-data; name=\"content\"; filename=\"demo_pkg-1.0.0-py3-none-any.whl\"\r\n\
         Content-Type: application/octet-stream\r\n\r\n{data}\r\n\
         --{b}--\r\n",
        b = boundary,
        sha = digest.trim_start_matches("sha256:"),
        data = String::from_utf8_lossy(wheel)
    );

    let upload = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/pypi/legacy/")
                .method("POST")
                .header(
                    "content-type",
                    format!("multipart/form-data; boundary={boundary}"),
                )
                .header("authorization", "Bearer adm")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        upload.status(),
        StatusCode::OK,
        "upload: {}",
        upload.status()
    );

    // Simple index (PEP 503 HTML — pip cũ/mới đều đọc được)
    let idx = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/pypi/simple/demo-pkg/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(idx.status(), StatusCode::OK, "index: {}", idx.status());
    let idx_body = axum::body::to_bytes(idx.into_body(), 8192).await.unwrap();
    let idx_html = String::from_utf8(idx_body.to_vec()).unwrap();
    assert!(
        idx_html.contains("Links for demo-pkg"),
        "index HTML: {idx_html}"
    );
    assert!(
        idx_html.contains("/pypi/packages/demo-pkg/demo_pkg-1.0.0-py3-none-any.whl"),
        "index link: {idx_html}"
    );

    // Download file
    let dl = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/pypi/packages/demo-pkg/demo_pkg-1.0.0-py3-none-any.whl")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(dl.status(), StatusCode::OK, "download: {}", dl.status());
    let dl_body = axum::body::to_bytes(dl.into_body(), 8192).await.unwrap();
    assert_eq!(&dl_body[..], wheel);

    // sha256 mismatch → rejected
    let bad = format!(
        "--{b}\r\nContent-Disposition: form-data; name=\"name\"\r\n\r\nbad-pkg\r\n\
         --{b}\r\nContent-Disposition: form-data; name=\"version\"\r\n\r\n1.0.0\r\n\
         --{b}\r\nContent-Disposition: form-data; name=\"sha256_digest\"\r\n\r\ndeadbeef\r\n\
         --{b}\r\nContent-Disposition: form-data; name=\"content\"; filename=\"bad.whl\"\r\n\r\nwhatever\r\n\
         --{b}--\r\n",
        b = boundary
    );
    let bad_req = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/pypi/legacy/")
                .method("POST")
                .header(
                    "content-type",
                    format!("multipart/form-data; boundary={boundary}"),
                )
                .header("authorization", "Bearer adm")
                .body(Body::from(bad))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        bad_req.status(),
        StatusCode::BAD_REQUEST,
        "sha mismatch rejected"
    );
}
