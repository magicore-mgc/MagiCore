use std::fs;
use tempfile;

use super::super::*;

#[tokio::test]
async fn automatic_fetch_uses_versioned_cache_for_registry_layers() {
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
        "web/frontend/vanilla",
        "web/shared/partials/base",
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
            let before_cache_check = server.requests.lock().unwrap().len();
            assert!(matches!(
                ensure_layer_in(&requested, &cache, Some(&server.url), Some(&auth))
                    .await
                    .unwrap(),
                ScaffoldResolveStatus::CacheHit { .. }
            ));
            let requests_after_cache_check = server.requests.lock().unwrap().len();
            let expected_requests = usize::from(suffix.is_empty() || suffix == "@latest");
            assert_eq!(
                requests_after_cache_check - before_cache_check,
                expected_requests,
                "moving selectors must check live metadata while exact versions reuse cache"
            );
        }
    }
}

#[tokio::test]
async fn default_template_selector_refreshes_when_registry_latest_moves() {
    use crate::scaffold::resolver::ScaffoldResolveStatus;
    use base64::Engine;
    use sha2::{Digest, Sha512};
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };

    let v2 = valid_archive();
    let v3 = archive(&[
        (
            "package/template.toml",
            b"[[files]]\nsource = 'main.txt'\ntarget = 'main.txt'\n",
            tar::EntryType::Regular,
        ),
        (
            "package/sources/main.txt",
            b"version three",
            tar::EntryType::Regular,
        ),
    ]);
    let integrity = |bytes: &[u8]| {
        format!(
            "sha512-{}",
            base64::engine::general_purpose::STANDARD.encode(Sha512::digest(bytes))
        )
    };
    let metadata = |latest: &str| {
        serde_json::to_vec(&serde_json::json!({
            "dist-tags": {"latest": latest},
            "versions": {
                "2.0.0": {"dist": {"tarball": "/v2.tgz", "integrity": integrity(&v2)}},
                "3.0.0": {"dist": {"tarball": "/v3.tgz", "integrity": integrity(&v3)}}
            }
        }))
        .unwrap()
    };
    let latest_moved = Arc::new(AtomicBool::new(false));
    let latest_moved_for_server = Arc::clone(&latest_moved);
    let metadata_v2 = metadata("2.0.0");
    let metadata_v3 = metadata("3.0.0");
    let server = HttpFixture::new(move |request| {
        let body = if request.starts_with("GET /v2.tgz ") {
            &v2
        } else if request.starts_with("GET /v3.tgz ") {
            &v3
        } else if latest_moved_for_server.load(Ordering::Relaxed) {
            &metadata_v3
        } else {
            &metadata_v2
        };
        http_response("200 OK", "", body)
    });
    let root = tempfile::tempdir().unwrap();
    let cache = root.path().canonicalize().unwrap();
    let auth = mgc_publish::auth::Auth::default();

    let first = ensure_layer_in(
        "web/frontend/vanilla",
        &cache,
        Some(&server.url),
        Some(&auth),
    )
    .await
    .unwrap();
    assert!(matches!(
        first,
        ScaffoldResolveStatus::Fetched { ref version, .. } if version == "2.0.0"
    ));

    latest_moved.store(true, Ordering::Relaxed);
    let refreshed = ensure_layer_in(
        "web/frontend/vanilla",
        &cache,
        Some(&server.url),
        Some(&auth),
    )
    .await
    .unwrap();
    assert!(matches!(
        refreshed,
        ScaffoldResolveStatus::Fetched { ref version, .. } if version == "3.0.0"
    ));
    let layer = cache.join("web/frontend/vanilla");
    assert_eq!(
        fs::read_to_string(layer.join("sources/main.txt")).unwrap(),
        "version three"
    );
    assert_eq!(
        read_template_cache_version(&layer).unwrap().as_deref(),
        Some("3.0.0")
    );
}

#[tokio::test]
async fn unversioned_web_template_cache_is_replaced_by_verified_registry_layer() {
    use crate::scaffold::resolver::ScaffoldResolveStatus;
    use base64::Engine;
    use sha2::{Digest, Sha512};

    let artifact = valid_archive();
    let integrity = format!(
        "sha512-{}",
        base64::engine::general_purpose::STANDARD.encode(Sha512::digest(&artifact))
    );
    let metadata = serde_json::to_vec(&serde_json::json!({
        "dist-tags": {"latest": "2.0.0"},
        "versions": {
            "2.0.0": {"dist": {"tarball": "/current.tgz", "integrity": integrity}}
        }
    }))
    .unwrap();
    let server = HttpFixture::new(move |request| {
        let body = if request.starts_with("GET /current.tgz ") {
            &artifact
        } else {
            &metadata
        };
        http_response("200 OK", "", body)
    });

    let root = tempfile::tempdir().unwrap();
    let cache = root.path().canonicalize().unwrap();
    let layer = cache.join("web/frontend/vanilla");
    fs::create_dir_all(layer.join("sources")).unwrap();
    fs::write(
        layer.join("template.toml"),
        "[[files]]\nsource = 'main.txt'\ntarget = 'main.txt'\n",
    )
    .unwrap();
    fs::write(layer.join("sources/main.txt"), "unversioned stale content").unwrap();

    let status = ensure_layer_in(
        "web/frontend/vanilla",
        &cache,
        Some(&server.url),
        Some(&mgc_publish::auth::Auth::default()),
    )
    .await
    .unwrap();

    assert!(matches!(
        status,
        ScaffoldResolveStatus::Fetched { ref version, .. } if version == "2.0.0"
    ));
    assert_eq!(
        fs::read_to_string(layer.join("sources/main.txt")).unwrap(),
        "new content"
    );
    assert_eq!(
        read_template_cache_version(&layer).unwrap().as_deref(),
        Some("2.0.0")
    );
    assert!(server.requests.lock().unwrap().len() >= 2);
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

#[tokio::test]
async fn core_layers_never_resolve_to_a_compiled_processor() {
    use crate::scaffold::resolver::ScaffoldResolveStatus;
    let root = tempfile::tempdir().unwrap();
    let auth = mgc_publish::auth::Auth::default();
    for rel in ["ai/python-agent", "app/flutter", "lib/rust"] {
        // No builtin processor may satisfy a default ref: an unreachable
        // registry is an availability failure, never an embedded layer.
        // Không processor builtin nào thỏa ref mặc định: registry không tới
        // được là lỗi khả dụng, không phải layer nhúng.
        let error = ensure_layer_in(rel, root.path(), None, Some(&auth))
            .await
            .expect_err("no compiled processor may stand in for a layer");
        assert!(error.is_availability_failure(), "{rel}: {error}");
        assert!(!root.path().join(rel).exists());
        let _: Option<ScaffoldResolveStatus> = None;
    }
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
}

#[tokio::test]
async fn non_directory_layer_cache_fails_closed_without_registry_fallback() {
    use crate::scaffold::resolver::ScaffoldResolveError;
    let root = tempfile::tempdir().unwrap();
    let corrupt_layer = root.path().join("web/shared/partials/base");
    fs::create_dir_all(corrupt_layer.parent().unwrap()).unwrap();
    fs::write(&corrupt_layer, b"not-a-template-directory").unwrap();
    let auth = mgc_publish::auth::Auth::default();

    let error = ensure_layer_in(
        "web/shared/partials/base",
        root.path(),
        Some("invalid registry must not be contacted"),
        Some(&auth),
    )
    .await
    .unwrap_err();

    assert!(matches!(error, ScaffoldResolveError::Other(_)));
    assert!(error.to_string().contains("refusing unsafe"));
    assert!(!error.is_availability_failure());
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
