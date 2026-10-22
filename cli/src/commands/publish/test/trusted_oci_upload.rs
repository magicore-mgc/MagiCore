//! Trusted OCI upload tests.
//! Kiểm thử upload OCI được ủy quyền.
use super::*;

#[derive(Default)]
struct MockOciRegistry {
    next_upload: u64,
    uploads: HashMap<String, (String, Vec<u8>)>,
    blobs: HashMap<(String, String), Vec<u8>>,
    manifest: Option<(String, String, serde_json::Value)>,
}

async fn mock_oci_registry(
    State(state): State<Arc<Mutex<MockOciRegistry>>>,
    AxumPath(path): AxumPath<String>,
    method: Method,
    headers: HeaderMap,
    Query(query): Query<HashMap<String, String>>,
    body: Bytes,
) -> Response {
    if headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        != Some("Bearer local-token")
    {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let route = path.as_str();
    let mut state = state.lock().unwrap();
    if method == Method::POST {
        let Some(repository) = route.strip_suffix("/blobs/uploads/") else {
            return StatusCode::NOT_FOUND.into_response();
        };
        state.next_upload += 1;
        let id = format!("upload-{}", state.next_upload);
        state
            .uploads
            .insert(id.clone(), (repository.to_owned(), Vec::new()));
        let mut response = StatusCode::ACCEPTED.into_response();
        response.headers_mut().insert(
            header::LOCATION,
            format!("/v2/{repository}/blobs/uploads/{id}")
                .parse()
                .unwrap(),
        );
        return response;
    }
    if method == Method::PATCH {
        let Some((_repository, upload_id)) = route.split_once("/blobs/uploads/") else {
            return StatusCode::NOT_FOUND.into_response();
        };
        let Some((_, bytes)) = state.uploads.get_mut(upload_id) else {
            return StatusCode::NOT_FOUND.into_response();
        };
        let Some((start, end)) = headers
            .get("content-range")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.split_once('-'))
        else {
            return StatusCode::RANGE_NOT_SATISFIABLE.into_response();
        };
        let (Ok(start), Ok(end)) = (start.parse::<usize>(), end.parse::<usize>()) else {
            return StatusCode::RANGE_NOT_SATISFIABLE.into_response();
        };
        if start != bytes.len()
            || end.checked_sub(start).and_then(|size| size.checked_add(1)) != Some(body.len())
        {
            return StatusCode::RANGE_NOT_SATISFIABLE.into_response();
        }
        bytes.extend_from_slice(&body);
        let mut response = StatusCode::ACCEPTED.into_response();
        response
            .headers_mut()
            .insert(header::LOCATION, format!("/v2/{route}").parse().unwrap());
        response.headers_mut().insert(
            header::RANGE,
            format!("0-{}", bytes.len() - 1).parse().unwrap(),
        );
        return response;
    }
    if method == Method::PUT && route.contains("/blobs/uploads/") {
        let Some((repository, upload_id)) = route.split_once("/blobs/uploads/") else {
            return StatusCode::NOT_FOUND.into_response();
        };
        let Some(expected_digest) = query.get("digest") else {
            return StatusCode::BAD_REQUEST.into_response();
        };
        let Some((_, bytes)) = state.uploads.remove(upload_id) else {
            return StatusCode::NOT_FOUND.into_response();
        };
        use sha2::Digest as _;
        let actual_digest = format!("sha256:{}", hex::encode(sha2::Sha256::digest(&bytes)));
        if &actual_digest != expected_digest {
            return StatusCode::BAD_REQUEST.into_response();
        }
        state
            .blobs
            .insert((repository.to_owned(), actual_digest), bytes);
        return StatusCode::CREATED.into_response();
    }
    if method == Method::PUT {
        let Some((repository, version)) = route.split_once("/manifests/") else {
            return StatusCode::NOT_FOUND.into_response();
        };
        let Ok(manifest) = serde_json::from_slice::<serde_json::Value>(&body) else {
            return StatusCode::BAD_REQUEST.into_response();
        };
        let config_digest = manifest["config"]["digest"].as_str().unwrap_or_default();
        if !state
            .blobs
            .contains_key(&(repository.to_owned(), config_digest.to_owned()))
        {
            return StatusCode::BAD_REQUEST.into_response();
        }
        if let Some(layers) = manifest["layers"].as_array() {
            for layer in layers {
                let digest = layer["digest"].as_str().unwrap_or_default();
                if !state
                    .blobs
                    .contains_key(&(repository.to_owned(), digest.to_owned()))
                {
                    return StatusCode::BAD_REQUEST.into_response();
                }
            }
        } else {
            return StatusCode::BAD_REQUEST.into_response();
        }
        state.manifest = Some((repository.to_owned(), version.to_owned(), manifest));
        return StatusCode::CREATED.into_response();
    }
    StatusCode::NOT_FOUND.into_response()
}

#[tokio::test]
async fn trusted_oci_artifact_upload_streams_blobs_and_publishes_manifest() {
    use sha2::Digest as _;

    let state = Arc::new(Mutex::new(MockOciRegistry::default()));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let app = Router::new()
        .route("/v2/{*path}", any(mock_oci_registry))
        .with_state(state.clone());
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let temp = tempfile::tempdir().unwrap();
    let artifact = temp.path().join("model.bin");
    let bytes = vec![0x5a; OCI_UPLOAD_CHUNK_BYTES * 2 + 17];
    fs::write(&artifact, &bytes).unwrap();
    let registry = format!("http://{address}");
    let mut session = super::TrustedPublishSession {
        token: "local-token".into(),
        expires_at: tokio::time::Instant::now() + std::time::Duration::from_secs(900),
        registry: registry.clone(),
        package: "ai/models/weights".into(),
        protocol: "oci".into(),
        audience: None,
    };
    let result = publish_trusted_oci_artifacts(
        &registry,
        "ai/models/weights",
        "ai",
        "1.0.0",
        &[artifact],
        &mut session,
    )
    .await;
    server.abort();
    result.unwrap();

    let state = state.lock().unwrap();
    assert_eq!(state.blobs.len(), 2);
    let (repository, version, manifest) = state.manifest.as_ref().unwrap();
    assert_eq!(repository, "ai/models/weights");
    assert_eq!(version, "1.0.0");
    assert_eq!(manifest["annotations"]["io.magicore.core"], "ai");
    assert_eq!(manifest["layers"][0]["size"], bytes.len() as u64);
    assert_eq!(
        manifest["layers"][0]["digest"],
        format!("sha256:{}", hex::encode(sha2::Sha256::digest(&bytes)))
    );
}
