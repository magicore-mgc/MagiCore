use super::super::*;

#[test]
fn template_credentials_follow_registry_origin_and_path_scope() {
    let registry = template_registry_url("https://registry.example/private/").unwrap();
    assert_eq!(
        template_registry_url("https://registry.example/private")
            .unwrap()
            .join("file.tgz")
            .unwrap()
            .as_str(),
        "https://registry.example/private/file.tgz"
    );
    for (target, allowed) in [
        ("https://registry.example/private/pkg.tgz", true),
        ("https://registry.example/private-other/pkg.tgz", false),
        ("https://registry.example/other/pkg.tgz", false),
        ("https://cdn.example/private/pkg.tgz", false),
        ("https://registry.example:4351/private/pkg.tgz", false),
        ("https://registry.example/private/%252e%252e/other", false),
    ] {
        assert_eq!(
            template_auth_allowed(&registry, &url::Url::parse(target).unwrap()),
            allowed,
            "{target}"
        );
    }
    validate_template_download_url(
        &url::Url::parse("https://cdn.example/file?signature=value").unwrap(),
        &registry,
    )
    .unwrap();
}

#[test]
fn template_download_urls_reject_credentials_and_plaintext_downgrades() {
    let registry = template_registry_url("https://registry.example/").unwrap();
    for value in [
        "file:///tmp/archive",
        "http://registry.example/file",
        "http://127.0.0.1:4315/file",
        "https://user:secret@example.com/file",
        "https://example.com/file#fragment",
    ] {
        let error = validate_template_download_url(&url::Url::parse(value).unwrap(), &registry)
            .unwrap_err();
        assert!(!error.to_string().contains("secret"));
    }
    template_registry_url("http://127.0.0.1:4315/private/").unwrap();
    assert!(template_registry_url("http://intranet.example/").is_err());
    assert!(template_registry_url("https://registry.example/?token=secret").is_err());
}

#[test]
fn template_integrity_requires_a_matching_strongest_digest() {
    use base64::Engine;
    use sha2::{Digest, Sha256, Sha384, Sha512};
    let body = b"archive payload";
    let sha256 = format!(
        "sha256-{}",
        base64::engine::general_purpose::STANDARD.encode(Sha256::digest(body))
    );
    let sha384 = format!(
        "sha384-{}",
        base64::engine::general_purpose::STANDARD.encode(Sha384::digest(body))
    );
    let sha512 = format!(
        "sha512-{}",
        base64::engine::general_purpose::STANDARD.encode(Sha512::digest(body))
    );
    for integrity in [&sha256, &sha384, &sha512] {
        verify_template_integrity(body, integrity).unwrap();
        assert!(verify_template_integrity(b"tampered", integrity).is_err());
    }
    let bad_sha512 = format!(
        "sha512-{}",
        base64::engine::general_purpose::STANDARD.encode(Sha512::digest(b"different"))
    );
    assert!(verify_template_integrity(body, &format!("{bad_sha512} {sha256}")).is_err());
    verify_template_integrity(body, &format!("{bad_sha512} {sha512}")).unwrap();
    verify_template_integrity(body, &format!("sha256-malformed {sha512}")).unwrap();
    verify_template_integrity(body, &sha512.replacen("sha512-", "SHA512-", 1)).unwrap();
    for integrity in ["", "sha1-YWJj", "sha512-not-base64", "sha256-YWJj"] {
        assert!(
            verify_template_integrity(body, integrity).is_err(),
            "{integrity}"
        );
    }
}

#[tokio::test]
async fn redirected_tarball_never_receives_registry_authorization() {
    let cdn = HttpFixture::new(|_| http_response("200 OK", "", b"artifact"));
    let location = format!("Location: {}/artifact\r\n", cdn.url);
    let registry = HttpFixture::new(move |_| http_response("302 Found", &location, b""));
    let registry_url = template_registry_url(&format!("{}/private/", registry.url)).unwrap();
    let result = download_template_bytes(
        &fixture_client(),
        registry_url.join("start").unwrap(),
        &registry_url,
        Some("Bearer fixture-token"),
        1024,
    )
    .await
    .unwrap();
    assert_eq!(result, b"artifact");
    let registry_requests = registry.requests.lock().unwrap().clone();
    assert!(
        registry_requests.first().is_some_and(|request| request
            .to_ascii_lowercase()
            .contains("authorization: bearer fixture-token")),
        "expected registry authorization on the first request, got: {registry_requests:#?}"
    );
    let cdn_requests = cdn.requests.lock().unwrap().clone();
    assert!(
        cdn_requests
            .first()
            .is_some_and(|request| !request.to_ascii_lowercase().contains("authorization:")),
        "expected an unauthenticated CDN request, got: {cdn_requests:#?}"
    );
}

#[tokio::test]
async fn body_without_content_length_is_still_bounded() {
    let server = HttpFixture::new(|_| http_response("200 OK", "", b"123456789"));
    let registry = template_registry_url(&server.url).unwrap();
    let error = download_template_bytes(&fixture_client(), registry.clone(), &registry, None, 8)
        .await
        .unwrap_err();
    assert!(
        error.to_string().contains("8 byte limit"),
        "expected response-size limit error, got: {error:#}"
    );
}

#[tokio::test]
async fn redirect_loops_stop_at_the_configured_limit() {
    let server = HttpFixture::new(|_| http_response("302 Found", "Location: /loop\r\n", b""));
    let registry = template_registry_url(&server.url).unwrap();
    assert!(
        download_template_bytes(&fixture_client(), registry.clone(), &registry, None, 8)
            .await
            .is_err()
    );
    assert_eq!(
        server.requests.lock().unwrap().len(),
        MAX_TEMPLATE_REDIRECTS + 1
    );
}
