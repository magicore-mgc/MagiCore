//! Template archive/cache boundary regression tests against the production extractor.
//! Kiểm thử archive độc hại và publish cache bằng đúng hàm production.

use super::*;

#[test]
fn publish_all_core_summary_lists_only_cores_with_template_layers() {
    let root = tempfile::tempdir().unwrap();
    let layers = vec![
        root.path().join("web/frontend/vanilla"),
        root.path().join("ai/agents/python-agent"),
        root.path().join("web/shared/partials/base"),
    ];

    assert_eq!(
        published_template_cores(root.path(), &layers).unwrap(),
        ["ai", "web"]
    );
}

#[test]
fn publish_all_rejects_a_layer_outside_the_template_root() {
    let root = tempfile::tempdir().unwrap();
    let outside = root.path().parent().unwrap().join("outside-template-layer");

    assert!(published_template_cores(root.path(), &[outside]).is_err());
}

fn archive(entries: &[(&str, &[u8], tar::EntryType)]) -> Vec<u8> {
    let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut builder = tar::Builder::new(encoder);
    for (path, contents, kind) in entries {
        let mut header = tar::Header::new_gnu();
        header.set_mode(0o644);
        header.set_size(contents.len() as u64);
        header.set_entry_type(*kind);
        // Raw headers intentionally bypass Builder's safe path setter for attack fixtures.
        // Header thô giữ nguyên path độc hại để kiểm boundary đọc archive.
        header.as_mut_bytes()[..path.len()].copy_from_slice(path.as_bytes());
        if kind.is_symlink() || kind.is_hard_link() {
            header.set_link_name("../../outside").unwrap();
        }
        header.set_cksum();
        builder.append(&header, *contents).unwrap();
    }
    builder.into_inner().unwrap().finish().unwrap()
}

fn valid_archive() -> Vec<u8> {
    archive(&[
        (
            "package/template.toml",
            b"[[files]]\nsource = 'main.txt'\ntarget = 'main.txt'\n",
            tar::EntryType::Regular,
        ),
        (
            "package/sources/main.txt",
            b"new content",
            tar::EntryType::Regular,
        ),
    ])
}

#[test]
fn valid_archive_replaces_cache_and_strips_the_package_prefix() {
    let root = tempfile::tempdir().unwrap();
    let root = root.path().canonicalize().unwrap();
    let target = root.join("layer");
    fs::create_dir(&target).unwrap();
    fs::write(target.join("old.txt"), b"old").unwrap();

    extract_tarball_into(&valid_archive(), &target, None).unwrap();

    assert!(target.join("template.toml").is_file());
    assert_eq!(
        fs::read(target.join("sources/main.txt")).unwrap(),
        b"new content"
    );
    assert!(!target.join("package").exists());
    assert!(!target.join("old.txt").exists());
    assert_eq!(fs::read_dir(&root).unwrap().count(), 1);
}

#[test]
fn traversal_archive_preserves_the_previous_cache() {
    let root = tempfile::tempdir().unwrap();
    let root = root.path().canonicalize().unwrap();
    let target = root.join("layer");
    fs::create_dir(&target).unwrap();
    fs::write(target.join("old.txt"), b"preserve").unwrap();
    let malicious = archive(&[
        (
            "package/template.toml",
            b"valid first entry",
            tar::EntryType::Regular,
        ),
        ("package/../../escaped", b"attack", tar::EntryType::Regular),
    ]);

    assert!(extract_tarball_into(&malicious, &target, None).is_err());
    assert_eq!(fs::read(target.join("old.txt")).unwrap(), b"preserve");
    assert!(!target.join("template.toml").exists());
    assert!(!root.join("escaped").exists());
    assert_eq!(fs::read_dir(&root).unwrap().count(), 1);
}

#[test]
fn invalid_manifest_does_not_replace_the_previous_cache() {
    let root = tempfile::tempdir().unwrap();
    let root = root.path().canonicalize().unwrap();
    let target = root.join("layer");
    fs::create_dir(&target).unwrap();
    fs::write(target.join("sentinel"), b"preserve").unwrap();
    let malformed = archive(&[
        (
            "package/template.toml",
            b"not = [valid",
            tar::EntryType::Regular,
        ),
        (
            "package/sources/main.txt",
            b"new content",
            tar::EntryType::Regular,
        ),
    ]);
    assert!(extract_tarball_into(&malformed, &target, None).is_err());
    assert_eq!(fs::read(target.join("sentinel")).unwrap(), b"preserve");
    assert_eq!(fs::read_dir(&target).unwrap().count(), 1);
}

#[test]
fn archive_links_and_multiple_roots_do_not_publish_a_cache() {
    for malicious in [
        archive(&[("package/sources/link", b"", tar::EntryType::Symlink)]),
        archive(&[("package/sources/link", b"", tar::EntryType::Link)]),
        archive(&[
            ("one/template.toml", b"one", tar::EntryType::Regular),
            ("two/template.toml", b"two", tar::EntryType::Regular),
        ]),
    ] {
        let root = tempfile::tempdir().unwrap();
        let root = root.path().canonicalize().unwrap();
        let target = root.join("layer");
        assert!(extract_tarball_into(&malicious, &target, None).is_err());
        assert!(!target.exists());
        assert_eq!(fs::read_dir(&root).unwrap().count(), 0);
    }
}

#[cfg(unix)]
#[test]
fn linked_cache_or_parent_does_not_modify_the_link_target() {
    use std::os::unix::fs::symlink;
    for linked_parent in [false, true] {
        let root = tempfile::tempdir().unwrap();
        let root = root.path().canonicalize().unwrap();
        let outside = root.join("outside");
        fs::create_dir(&outside).unwrap();
        fs::write(outside.join("sentinel"), b"preserve").unwrap();
        let link = root.join("link");
        symlink(&outside, &link).unwrap();
        let target = if linked_parent {
            link.join("layer")
        } else {
            link
        };

        assert!(extract_tarball_into(&valid_archive(), &target, None).is_err());
        assert_eq!(fs::read(outside.join("sentinel")).unwrap(), b"preserve");
        assert_eq!(fs::read_dir(&outside).unwrap().count(), 1);
    }
}

#[test]
fn layer_identity_rejects_paths_that_escape_the_cache_on_any_platform() {
    for name in [
        "",
        "../layer",
        "/absolute",
        "a/../../b",
        "a//b",
        "C:\\layer",
        "a\\b",
        "a:b",
        "a\nb",
    ] {
        assert!(validate_template_identity("web", name).is_err(), "{name:?}");
    }
    assert!(validate_template_identity("unknown", "layer").is_err());
    for (core, name) in [
        ("web", "web/backend/go/gin"),
        ("ai", "python-agent"),
        ("app", "flutter"),
        ("lib", "rust"),
    ] {
        validate_template_identity(core, name).unwrap();
    }
}

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

struct HttpFixture {
    url: String,
    requests: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
    stop: std::sync::Arc<std::sync::atomic::AtomicBool>,
    worker: Option<std::thread::JoinHandle<()>>,
}

impl HttpFixture {
    fn new(respond: impl Fn(&str) -> Vec<u8> + Send + 'static) -> Self {
        use std::io::{Read, Write};
        use std::sync::{
            Arc, Mutex,
            atomic::{AtomicBool, Ordering},
        };
        const PORTS: &[u16] = &[
            5134, 5143, 5314, 5341, 5413, 5431, 4315, 4351, 4135, 4153, 4513, 4531, 3415, 3451,
            3145, 3154, 3541, 3514, 1345, 1354, 1435, 1453, 1534, 1543,
        ];
        let listener = PORTS
            .iter()
            .find_map(|port| {
                std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, *port)).ok()
            })
            .expect("an isolated permitted fixture port must be available");
        let url = format!("http://{}", listener.local_addr().unwrap());
        listener.set_nonblocking(true).unwrap();
        let requests = Arc::new(Mutex::new(Vec::new()));
        let stop = Arc::new(AtomicBool::new(false));
        let captured = requests.clone();
        let finished = stop.clone();
        let worker = std::thread::spawn(move || {
            while !finished.load(Ordering::Relaxed) {
                let (mut stream, _) = match listener.accept() {
                    Ok(connection) => connection,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(std::time::Duration::from_millis(5));
                        continue;
                    }
                    Err(_) => break,
                };
                // The listener is nonblocking for the accept loop. Explicitly
                // restore blocking mode on each accepted socket: some OSes
                // propagate O_NONBLOCK, which would make the first read return
                // WouldBlock before the HTTP client has written its request.
                stream.set_nonblocking(false).unwrap();
                stream
                    .set_read_timeout(Some(std::time::Duration::from_secs(2)))
                    .unwrap();
                stream
                    .set_write_timeout(Some(std::time::Duration::from_secs(2)))
                    .unwrap();
                let mut request = Vec::new();
                let mut buffer = [0u8; 1024];
                while request.len() < 16 * 1024
                    && !request.windows(4).any(|part| part == b"\r\n\r\n")
                {
                    match stream.read(&mut buffer) {
                        Ok(0) | Err(_) => break,
                        Ok(count) => request.extend_from_slice(&buffer[..count]),
                    }
                }
                if let Some(end) = request.windows(4).position(|part| part == b"\r\n\r\n") {
                    request.truncate(end + 4);
                }
                if request.is_empty() {
                    continue;
                }
                let request = String::from_utf8(request).unwrap();
                captured.lock().unwrap().push(request.clone());
                let _ = stream.write_all(&respond(&request));
            }
        });
        Self {
            url,
            requests,
            stop,
            worker: Some(worker),
        }
    }
}

impl Drop for HttpFixture {
    fn drop(&mut self) {
        self.stop.store(true, std::sync::atomic::Ordering::Relaxed);
        self.worker.take().unwrap().join().unwrap();
    }
}

fn http_response(status: &str, extra_headers: &str, body: &[u8]) -> Vec<u8> {
    let mut response =
        format!("HTTP/1.1 {status}\r\nConnection: close\r\n{extra_headers}\r\n").into_bytes();
    response.extend_from_slice(body);
    response
}

fn fixture_client() -> reqwest::Client {
    reqwest::Client::builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .unwrap()
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

#[test]
fn nested_layer_identity_round_trips_to_the_processor_cache() {
    assert_eq!(
        layer_rel("web", "backend/node/probe"),
        "web/backend/node/probe"
    );
    assert_eq!(
        registry_package_name("web", "web/backend/node/probe"),
        "mgc-create-web-backend-node-probe"
    );
}

#[test]
fn only_availability_errors_allow_scaffold_fallback() {
    for status in [401, 403, 404, 410, 429, 503] {
        let error = template_resolution_error(
            crate::error::template_download_status(status),
            "ai/probe",
            "ai",
            "probe",
            "latest",
        );
        assert_eq!(
            error.is_availability_failure(),
            matches!(status, 404 | 410 | 429 | 503)
        );
    }
    let error = template_resolution_error(
        crate::error::template_integrity_mismatch(),
        "lib/probe",
        "lib",
        "probe",
        "latest",
    );
    assert!(!error.is_availability_failure());
}

#[tokio::test]
async fn automatic_fetch_uses_visible_cache_and_honors_versions_for_four_cores() {
    use crate::scaffold::resolver::ScaffoldResolveStatus;
    use base64::Engine;
    use sha2::{Digest, Sha512};
    let older = valid_archive();
    let newer = archive(&[
        (
            "package/template.toml",
            b"[[files]]\nsource = 'main.txt'\ntarget = 'main.txt'\n",
            tar::EntryType::Regular,
        ),
        (
            "package/sources/main.txt",
            b"version two",
            tar::EntryType::Regular,
        ),
    ]);
    let integrity = |bytes: &[u8]| {
        format!(
            "sha512-{}",
            base64::engine::general_purpose::STANDARD.encode(Sha512::digest(bytes))
        )
    };
    let metadata = serde_json::to_vec(&serde_json::json!({
        "dist-tags": {"latest": "2.0.0"},
        "versions": {
            "1.0.0": {"dist": {"tarball": "/one.tgz", "integrity": integrity(&older)}},
            "2.0.0": {"dist": {"tarball": "/two.tgz", "integrity": integrity(&newer)}}
        }
    }))
    .unwrap();
    let server = HttpFixture::new(move |request| {
        let body = if request.starts_with("GET /one.tgz ") {
            &older
        } else if request.starts_with("GET /two.tgz ") {
            &newer
        } else {
            &metadata
        };
        http_response("200 OK", "", body)
    });
    let root = tempfile::tempdir().unwrap();
    let cache = root.path().canonicalize().unwrap();
    let auth = mgc_publish::auth::Auth::default();
    for rel in [
        "web/backend/node/mgc-fixture-layer",
        "ai/mgc-fixture-layer",
        "app/mgc-fixture-layer",
        "lib/mgc-fixture-layer",
    ] {
        for (suffix, version, content) in [
            ("", "2.0.0", b"version two".as_slice()),
            ("@1.0.0", "1.0.0", b"new content".as_slice()),
            ("@latest", "2.0.0", b"version two".as_slice()),
        ] {
            let requested = format!("{rel}{suffix}");
            let result = ensure_layer_in(&requested, &cache, Some(&server.url), Some(&auth))
                .await
                .unwrap_or_else(|error| {
                    let requests = server.requests.lock().unwrap().clone();
                    panic!(
                        "ensure_layer_in({requested:?}) failed: {error:#}; fixture recorded {} requests: {requests:#?}",
                        requests.len()
                    );
                });
            match result {
                ScaffoldResolveStatus::Fetched {
                    path,
                    version: actual,
                    ..
                } => {
                    assert_eq!(path, cache.join(rel));
                    assert_eq!(actual, version);
                    assert_eq!(fs::read(path.join("sources/main.txt")).unwrap(), content);
                    assert_eq!(
                        read_template_cache_version(&path).unwrap().as_deref(),
                        Some(version)
                    );
                }
                other => panic!("expected fetched layer, got {other:?}"),
            }
            if suffix != "@latest" {
                let count = server.requests.lock().unwrap().len();
                assert!(matches!(
                    ensure_layer_in(&requested, &cache, Some(&server.url), Some(&auth))
                        .await
                        .unwrap(),
                    ScaffoldResolveStatus::CacheHit { .. }
                ));
                assert_eq!(server.requests.lock().unwrap().len(), count);
            }
        }
    }
}

#[tokio::test]
async fn automatic_fetch_integrity_failure_never_publishes_or_allows_fallback() {
    let bytes = valid_archive();
    let metadata = serde_json::to_vec(&serde_json::json!({
        "dist-tags": {"latest": "1.0.0"},
        "versions": {"1.0.0": {"dist": {"tarball": "/bad.tgz", "integrity": "sha512-invalid"}}}
    }))
    .unwrap();
    let server = HttpFixture::new(move |request| {
        http_response(
            "200 OK",
            "",
            if request.starts_with("GET /bad.tgz ") {
                &bytes
            } else {
                &metadata
            },
        )
    });
    let root = tempfile::tempdir().unwrap();
    let auth = mgc_publish::auth::Auth::default();
    for core in ["web", "ai", "app", "lib"] {
        let rel = format!("{core}/mgc-integrity-fixture");
        let error = ensure_layer_in(&rel, root.path(), Some(&server.url), Some(&auth))
            .await
            .unwrap_err();
        assert!(!error.is_availability_failure());
        assert!(!root.path().join(rel).exists());
    }
}

#[test]
fn version_marker_alone_is_not_a_usable_layer() {
    let root = tempfile::tempdir().unwrap();
    fs::write(root.path().join(".mgc-version"), "1.0.0").unwrap();
    assert!(!template_layer_has_contract(root.path()).unwrap());
    fs::write(
        root.path().join("template.toml"),
        "[[files]]\nsource = 'main.txt'\ntarget = 'main.txt'\n",
    )
    .unwrap();
    assert!(!template_layer_has_contract(root.path()).unwrap());
    fs::create_dir(root.path().join("sources")).unwrap();
    assert!(template_layer_has_contract(root.path()).unwrap());
}

#[test]
fn syntactically_valid_toml_without_processor_schema_preserves_existing_cache() {
    let root = tempfile::tempdir().unwrap();
    let target = root.path().join("layer");
    fs::create_dir(&target).unwrap();
    fs::write(target.join("sentinel"), "preserve").unwrap();
    let bytes = archive(&[
        (
            "package/template.toml",
            b"[template]\nname = 'unusable'\n",
            tar::EntryType::Regular,
        ),
        (
            "package/sources/main.txt",
            b"content",
            tar::EntryType::Regular,
        ),
    ]);
    assert!(extract_tarball_into(&bytes, &target, Some("1.0.0")).is_err());
    assert_eq!(
        fs::read_to_string(target.join("sentinel")).unwrap(),
        "preserve"
    );
    assert!(!target.join(".mgc-version").exists());
    fs::write(
        target.join("template.toml"),
        "[template]\nname = 'unusable'\n",
    )
    .unwrap();
    fs::create_dir(target.join("sources")).unwrap();
    assert!(template_layer_has_contract(&target).is_err());
}

#[test]
fn every_active_embedded_layer_has_a_processor_compatible_contract() {
    use crate::scaffold::embedded::EmbeddedKernel;
    let root = tempfile::tempdir().unwrap();
    for rel in EmbeddedKernel::list_available() {
        let target = root.path().join(&rel);
        EmbeddedKernel::extract_layer_path(&rel, &target).unwrap();
        assert!(template_layer_has_contract(&target).unwrap(), "{rel}");
    }
    assert_eq!(
        EmbeddedKernel::layer_version_path("web/vanilla"),
        EmbeddedKernel::layer_version_path("web/frontend/vanilla")
    );
}

#[tokio::test]
async fn compiled_core_processors_do_not_create_unused_template_snapshot_directories() {
    use crate::scaffold::resolver::ScaffoldResolveStatus;
    let root = tempfile::tempdir().unwrap();
    let auth = mgc_publish::auth::Auth::default();
    for rel in ["ai/python-agent", "app/flutter", "lib/rust"] {
        let status = ensure_layer_in(
            rel,
            root.path(),
            Some("invalid registry must not be contacted"),
            Some(&auth),
        )
        .await
        .unwrap();
        assert!(matches!(status, ScaffoldResolveStatus::Embedded { .. }));
        assert!(!root.path().join(rel).exists());
    }
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
}

#[test]
fn normalized_create_specs_keep_explicit_template_refs_for_all_core_kinds() {
    use crate::scaffold::spec::{CoreKind, parse_scaffold_spec};
    for core in [
        CoreKind::Web,
        CoreKind::Ai,
        CoreKind::App,
        CoreKind::Lib,
        CoreKind::Cloud,
        CoreKind::Game,
        CoreKind::Iot,
        CoreKind::Cicd,
        CoreKind::Hardware,
    ] {
        for reference in ["", "@1.2.3", "@latest", "@stable"] {
            let spec = parse_scaffold_spec(core, &format!("Fixture_Layer{reference}")).unwrap();
            assert_eq!(spec.template_request(), format!("fixture-layer{reference}"));
        }
    }
}

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
