use std::fs;
use tempfile;

use super::super::*;

#[test]
fn publish_urls_preserve_registry_prefix_and_encode_path_data() {
    let registry = template_registry_url("https://registry.example/private/").unwrap();
    let target = template_publish_url(
        &registry,
        "mgc-create-lib-probe?query#fragment",
        Some("1.2.3"),
    )
    .unwrap();
    assert_eq!(target.origin(), registry.origin());
    assert!(
        target
            .path()
            .starts_with("/private/mgc-create-lib-probe%3Fquery%23fragment/-/")
    );
    assert!(target.query().is_none());
    assert!(target.fragment().is_none());
    assert!(template_publish_url(&registry, "probe", Some("../escape")).is_err());
}

#[tokio::test]
async fn publish_redirect_does_not_forward_credentials_or_write_body() {
    let destination = HttpFixture::new(|_| http_response("201 Created", "", b""));
    let location = format!("Location: {}/receive\r\n", destination.url);
    let registry = HttpFixture::new(move |_| {
        http_response("307 Temporary Redirect", &location, b"secret-response")
    });
    let base = template_registry_url(&registry.url).unwrap();
    let target = template_publish_url(&base, "mgc-create-ai-probe", None).unwrap();
    let auth = mgc_publish::auth::Auth {
        token: Some("fixture-token".to_string()),
        ..Default::default()
    };
    let error = put_template_bytes(&base, target, &auth, b"{}".to_vec(), true)
        .await
        .unwrap_err();
    assert!(error.to_string().contains("HTTP 307"));
    assert!(!error.to_string().contains("secret-response"));
    assert!(!error.to_string().contains("fixture-token"));
    assert_eq!(registry.requests.lock().unwrap().len(), 1);
    assert!(destination.requests.lock().unwrap().is_empty());
}

#[tokio::test]
async fn publish_success_requires_a_successful_status_and_scoped_target() {
    let server = HttpFixture::new(|_| http_response("201 Created", "", b""));
    let registry = template_registry_url(&format!("{}/private/", server.url)).unwrap();
    let target = template_publish_url(&registry, "mgc-create-app-probe", None).unwrap();
    let auth = mgc_publish::auth::Auth::default();
    put_template_bytes(&registry, target, &auth, b"{}".to_vec(), true)
        .await
        .unwrap();
    let requests = server.requests.lock().unwrap().clone();
    assert!(
        requests
            .first()
            .is_some_and(|request| request.starts_with("PUT /private/mgc-create-app-probe ")),
        "expected PUT to the scoped package path, got: {requests:#?}"
    );
    let outside = registry.join("/outside").unwrap();
    assert!(
        put_template_bytes(&registry, outside, &auth, vec![], false)
            .await
            .is_err()
    );
    assert_eq!(server.requests.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn upload_tarball_uses_the_same_validated_publish_boundary() {
    let server = HttpFixture::new(|_| http_response("201 Created", "", b""));
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("probe.tgz");
    fs::write(&path, b"\xff\x00\x80").unwrap();
    upload_tarball(
        &server.url,
        &mgc_publish::auth::Auth::default(),
        "mgc-create-web-probe",
        "1.0.0",
        &path,
    )
    .await
    .unwrap();
    assert!(
        server.requests.lock().unwrap()[0]
            .starts_with("PUT /mgc-create-web-probe/-/mgc-create-web-probe-1.0.0.tgz ")
    );
}
