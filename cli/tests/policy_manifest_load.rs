//! Policy manifest loading tests — kiểm thử nạp manifest policy.
//!
//! Project-local policy data cannot authorize its own signer through
//! `mgc.toml`; trust roots are supplied explicitly by the user.
//! (Manifest `.mgc/policy/exceptions.json` của project cộng
//! project không thể tự cấp quyền manifest; trust root phải do user cấu hình.)

#![allow(clippy::unwrap_used)]

use mgc::commands::policy::load_active_exception_manifest;

fn signed_manifest_json(id: &str, issued_at: i64) -> (String, String) {
    signed_manifest_json_with_revocations(id, issued_at, vec![], [9u8; 32])
}

fn signed_manifest_json_with_revocations(
    id: &str,
    issued_at: i64,
    revoked_keys: Vec<String>,
    seed: [u8; 32],
) -> (String, String) {
    signed_manifest_json_with_expiry(id, issued_at, 9_000_000_000, revoked_keys, seed)
}

fn signed_manifest_json_with_expiry(
    id: &str,
    issued_at: i64,
    expires_at: i64,
    revoked_keys: Vec<String>,
    seed: [u8; 32],
) -> (String, String) {
    let signer = mgc_crypto::ed25519_signer::Ed25519Signer::from_seed(&seed).unwrap();
    let key_id = signer.public_key().to_base64();
    let manifest = mgc_exec::policy_manifest::PolicyManifest {
        schema_version: mgc_exec::policy_manifest::POLICY_MANIFEST_SCHEMA.to_string(),
        id: id.to_string(),
        issued_at,
        expires_at,
        key_id: key_id.clone(),
        revoked_keys,
        entries: vec![],
        signature: String::new(),
    };
    let bytes = mgc_exec::policy_manifest::canonical_bytes(&manifest);
    let signature = signer.sign_string(&bytes).to_base64();
    let text = serde_json::json!({
        "schema_version": manifest.schema_version,
        "id": manifest.id,
        "issued_at": manifest.issued_at,
        "expires_at": manifest.expires_at,
        "key_id": manifest.key_id,
        "revoked_keys": manifest.revoked_keys,
        "entries": manifest.entries,
        "signature": signature,
    });
    let hex = mgc_exec::policy_manifest::key_id_hex(&key_id).unwrap();
    (serde_json::to_string(&text).unwrap(), hex)
}

// Signed project manifest + matching trust root loads.
// (Manifest project đã ký + trust root khớp thì nạp được.)
#[test]
fn project_trust_root_does_not_authorize_manifest() {
    let dir = tempfile::TempDir::new().unwrap();
    let (text, hex) = signed_manifest_json("proj-1", 1_000_000);
    std::fs::write(
        dir.path().join("mgc.toml"),
        format!("name = \"p\"\necosystem = \"web\"\n[trust]\nkeys = [\"{hex}\"]\n"),
    )
    .unwrap();
    let policy_dir = dir.path().join(".magicore").join("policy");
    std::fs::create_dir_all(&policy_dir).unwrap();
    std::fs::write(policy_dir.join("exceptions.json"), text).unwrap();

    assert!(load_active_exception_manifest(dir.path()).is_none());
}

// Trust mismatch means no manifest (native-deny stays).
// (Trust lệch nghĩa là không manifest (giữ native-deny).)
#[test]
fn trust_mismatch_yields_none() {
    let dir = tempfile::TempDir::new().unwrap();
    let (text, _) = signed_manifest_json("proj-1", 1_000_000);
    std::fs::write(
        dir.path().join("mgc.toml"),
        "name = \"p\"\necosystem = \"web\"\n[trust]\nkeys = [\"deadbeefdeadbeef\"]\n",
    )
    .unwrap();
    let policy_dir = dir.path().join(".magicore").join("policy");
    std::fs::create_dir_all(&policy_dir).unwrap();
    std::fs::write(policy_dir.join("exceptions.json"), text).unwrap();

    assert!(load_active_exception_manifest(dir.path()).is_none());
}

// No trust roots configured means no manifest.
// (Không cấu hình trust roots nghĩa là không manifest.)
#[test]
fn no_trust_roots_yields_none() {
    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("mgc.toml"),
        "name = \"p\"\necosystem = \"web\"\n",
    )
    .unwrap();
    assert!(load_active_exception_manifest(dir.path()).is_none());
}

// URL distribution: fetch → verify → atomically cache (loopback HTTP only
// in tests; production requires HTTPS).
// (Phân phối URL: tải → xác minh → cache atomically (test chỉ HTTP loopback;
// production yêu cầu HTTPS).)
#[tokio::test]
async fn url_refresh_verifies_and_caches() {
    let mut server = mockito::Server::new_async().await;
    let (text, hex) = signed_manifest_json("url-1", 1_000_000);
    let value: serde_json::Value = serde_json::from_str(&text).unwrap();
    let _mock = server
        .mock("GET", "/policy.json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(value.to_string())
        .create_async()
        .await;

    let dir = tempfile::TempDir::new().unwrap();
    let cache = dir.path().join("policy").join("exceptions.json");
    let url = format!("{}/policy.json", server.url());
    let loaded = mgc::commands::policy::refresh_manifest_from_url(&url, &[hex], &cache)
        .await
        .unwrap();
    assert_eq!(loaded.id, "url-1");
    assert!(cache.exists());
    let cached: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&cache).unwrap()).unwrap();
    assert_eq!(cached["id"], "url-1");
}

// A stale distribution response cannot replace a newer verified cache.
// Snapshot cũ từ kênh phân phối không được ghi đè cache mới hơn đã xác minh.
#[tokio::test]
async fn url_refresh_rejects_older_snapshot_without_replacing_cache() {
    let mut server = mockito::Server::new_async().await;
    let (newer, trusted_hex) = signed_manifest_json("newer", 2_000_000);
    let (older, _) = signed_manifest_json("older", 1_000_000);
    let newer_value: serde_json::Value = serde_json::from_str(&newer).unwrap();
    let older_value: serde_json::Value = serde_json::from_str(&older).unwrap();
    let _newer_mock = server
        .mock("GET", "/newer.json")
        .with_status(200)
        .with_body(newer_value.to_string())
        .create_async()
        .await;
    let _older_mock = server
        .mock("GET", "/older.json")
        .with_status(200)
        .with_body(older_value.to_string())
        .create_async()
        .await;

    let dir = tempfile::TempDir::new().unwrap();
    let cache = dir.path().join("policy").join("exceptions.json");
    let base_url = server.url();
    mgc::commands::policy::refresh_manifest_from_url(
        &format!("{base_url}/newer.json"),
        std::slice::from_ref(&trusted_hex),
        &cache,
    )
    .await
    .unwrap();
    let err = mgc::commands::policy::refresh_manifest_from_url(
        &format!("{base_url}/older.json"),
        &[trusted_hex],
        &cache,
    )
    .await
    .unwrap_err();

    assert!(err.to_string().contains("older"), "got: {err}");
    let cached: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&cache).unwrap()).unwrap();
    assert_eq!(cached["id"], "newer");
}

// A newer snapshot must retain every verified revocation already cached.
// Snapshot mới phải giữ lại toàn bộ khóa đã bị thu hồi trong cache đã xác minh.
#[tokio::test]
async fn url_refresh_rejects_dropped_revocations_without_replacing_cache() {
    let mut server = mockito::Server::new_async().await;
    let retired_signer = mgc_crypto::ed25519_signer::Ed25519Signer::from_seed(&[10u8; 32]).unwrap();
    let retired_key =
        mgc_exec::policy_manifest::key_id_hex(&retired_signer.public_key().to_base64()).unwrap();
    let (old, trusted_hex) = signed_manifest_json_with_revocations(
        "with-revocation",
        1_000_000,
        vec![retired_key.clone()],
        [9u8; 32],
    );
    let (new, _) = signed_manifest_json("drops-revocation", 2_000_000);
    let (retained, _) = signed_manifest_json_with_revocations(
        "retains-revocation",
        3_000_000,
        vec![retired_key.clone()],
        [9u8; 32],
    );
    let old_value: serde_json::Value = serde_json::from_str(&old).unwrap();
    let new_value: serde_json::Value = serde_json::from_str(&new).unwrap();
    let retained_value: serde_json::Value = serde_json::from_str(&retained).unwrap();
    let _old_mock = server
        .mock("GET", "/old.json")
        .with_status(200)
        .with_body(old_value.to_string())
        .create_async()
        .await;
    let _new_mock = server
        .mock("GET", "/new.json")
        .with_status(200)
        .with_body(new_value.to_string())
        .create_async()
        .await;
    let _retained_mock = server
        .mock("GET", "/retained.json")
        .with_status(200)
        .with_body(retained_value.to_string())
        .create_async()
        .await;

    let dir = tempfile::TempDir::new().unwrap();
    let cache = dir.path().join("policy").join("exceptions.json");
    let base_url = server.url();
    mgc::commands::policy::refresh_manifest_from_url(
        &format!("{base_url}/old.json"),
        std::slice::from_ref(&trusted_hex),
        &cache,
    )
    .await
    .unwrap();
    let err = mgc::commands::policy::refresh_manifest_from_url(
        &format!("{base_url}/new.json"),
        std::slice::from_ref(&trusted_hex),
        &cache,
    )
    .await
    .unwrap_err();

    assert!(err.to_string().contains(&retired_key), "got: {err}");
    let cached: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&cache).unwrap()).unwrap();
    assert_eq!(cached["id"], "with-revocation");
    assert_eq!(cached["revoked_keys"][0], retired_key);

    mgc::commands::policy::refresh_manifest_from_url(
        &format!("{base_url}/retained.json"),
        std::slice::from_ref(&trusted_hex),
        &cache,
    )
    .await
    .unwrap();
    let cached: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&cache).unwrap()).unwrap();
    assert_eq!(cached["id"], "retains-revocation");
    assert_eq!(cached["revoked_keys"][0], retired_key);
}

// An expired cache still carries signed revocations, though it grants no
// exception. (Cache hết hạn vẫn giữ revocation có ký, nhưng không cấp exception.)
#[tokio::test]
async fn expired_cache_revocations_cannot_be_removed_by_refresh() {
    let mut server = mockito::Server::new_async().await;
    let retired_signer = mgc_crypto::ed25519_signer::Ed25519Signer::from_seed(&[10u8; 32]).unwrap();
    let retired_key =
        mgc_exec::policy_manifest::key_id_hex(&retired_signer.public_key().to_base64()).unwrap();
    let (expired, trusted_hex) = signed_manifest_json_with_expiry(
        "expired-with-revocation",
        1_000_000,
        1_500_000,
        vec![retired_key.clone()],
        [9u8; 32],
    );
    let (new, _) = signed_manifest_json("drops-expired-revocation", 2_000_000);
    let new_value: serde_json::Value = serde_json::from_str(&new).unwrap();
    let _new_mock = server
        .mock("GET", "/new.json")
        .with_status(200)
        .with_body(new_value.to_string())
        .create_async()
        .await;

    let dir = tempfile::TempDir::new().unwrap();
    let cache = dir.path().join("policy").join("exceptions.json");
    std::fs::create_dir_all(cache.parent().unwrap()).unwrap();
    std::fs::write(&cache, expired).unwrap();
    let err = mgc::commands::policy::refresh_manifest_from_url(
        &format!("{}/new.json", server.url()),
        &[trusted_hex],
        &cache,
    )
    .await
    .unwrap_err();

    assert!(err.to_string().contains(&retired_key), "got: {err}");
    let cached: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&cache).unwrap()).unwrap();
    assert_eq!(cached["id"], "expired-with-revocation");
}

// Plain HTTP outside loopback is refused before any request.
// (HTTP thường ngoài loopback bị từ chối trước mọi request.)
#[tokio::test]
async fn url_refresh_refuses_plain_http() {
    let dir = tempfile::TempDir::new().unwrap();
    let cache = dir.path().join("exceptions.json");
    let err = mgc::commands::policy::refresh_manifest_from_url(
        "http://registry.example.invalid/policy.json",
        &["deadbeefdeadbeef".to_string()],
        &cache,
    )
    .await
    .unwrap_err();
    assert!(err.to_string().contains("HTTPS"), "got: {err}");
    assert!(!cache.exists());
}

// OCI reference parsing: registry, repo, tag (default latest).
// (Parse reference OCI: registry, repo, tag (mặc định latest).)
#[test]
fn oci_reference_parsing() {
    use mgc::commands::policy::parse_oci_reference;
    assert_eq!(
        parse_oci_reference("oci://registry.example.com/magicore/policy:exceptions"),
        Some((
            "https://registry.example.com".to_string(),
            "magicore/policy".to_string(),
            "exceptions".to_string()
        ))
    );
    assert_eq!(
        parse_oci_reference("oci://127.0.0.1:5000/p:1"),
        Some((
            "http://127.0.0.1:5000".to_string(),
            "p".to_string(),
            "1".to_string()
        ))
    );
    assert_eq!(
        parse_oci_reference("oci://registry.example.com/p"),
        Some((
            "https://registry.example.com".to_string(),
            "p".to_string(),
            "latest".to_string()
        ))
    );
    assert!(parse_oci_reference("https://registry.example.com/p").is_none());
    assert!(parse_oci_reference("oci://onlyhost").is_none());
}

// OCI distribution round-trip against a mocked registry: manifest →
// config-blob bytes → verified cache. Layers are refused.
// (Round-trip phân phối OCI với registry giả: manifest → byte config-blob →
// cache đã xác minh. Layers bị từ chối.)
#[tokio::test]
async fn oci_refresh_roundtrip() {
    use mgc::commands::policy::refresh_manifest_from_oci;

    let mut server = mockito::Server::new_async().await;
    let (text, hex) = signed_manifest_json("oci-1", 1_000_000);
    let digest = format!("sha256:{}", sha256_hex(text.as_bytes()));
    let manifest = serde_json::json!({
        "schemaVersion": 2,
        "mediaType": "application/vnd.oci.image.manifest.v1+json",
        "config": {
            "mediaType": "application/vnd.magicore.policy.exceptions.v2+json",
            "size": text.len(),
            "digest": digest,
        },
        "layers": [],
    });
    let _manifest_mock = server
        .mock("GET", "/v2/p/manifests/exceptions")
        .with_status(200)
        .with_header("content-type", "application/vnd.oci.image.manifest.v1+json")
        .with_body(manifest.to_string())
        .create_async()
        .await;
    let blob_path = format!("/v2/p/blobs/{digest}");
    let _head_mock = server
        .mock("HEAD", blob_path.as_str())
        .with_status(200)
        .create_async()
        .await;
    let _blob_mock = server
        .mock("GET", blob_path.as_str())
        .with_status(200)
        .with_body(text.clone())
        .create_async()
        .await;

    let dir = tempfile::TempDir::new().unwrap();
    let cache = dir.path().join("exceptions.json");
    let reference = format!("{}/p:exceptions", server.url().replace("http://", "oci://"));
    let loaded = refresh_manifest_from_oci(&reference, &[hex], &cache)
        .await
        .unwrap();
    assert_eq!(loaded.id, "oci-1");
    assert!(cache.exists());
}

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::Digest;
    hex::encode(sha2::Sha256::digest(bytes))
}

// Wrong media type and non-empty layers are refused.
// (Sai media type và layers không rỗng bị từ chối.)
#[tokio::test]
async fn oci_refresh_refuses_layers() {
    use mgc::commands::policy::refresh_manifest_from_oci;

    let mut server = mockito::Server::new_async().await;
    let manifest = serde_json::json!({
        "schemaVersion": 2,
        "mediaType": "application/vnd.oci.image.manifest.v1+json",
        "config": {
            "mediaType": "application/vnd.magicore.policy.exceptions.v2+json",
            "size": 1,
            "digest": "sha256:deadbeef",
        },
        "layers": [{"mediaType": "application/octet-stream", "size": 1, "digest": "sha256:deadbeef"}],
    });
    let _manifest_mock = server
        .mock("GET", "/v2/p/manifests/latest")
        .with_status(200)
        .with_body(manifest.to_string())
        .create_async()
        .await;

    let dir = tempfile::TempDir::new().unwrap();
    let cache = dir.path().join("exceptions.json");
    let reference = format!("{}/p", server.url().replace("http://", "oci://"));
    let err = refresh_manifest_from_oci(&reference, &["deadbeefdeadbeef".to_string()], &cache)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("layers"), "got: {err}");
    assert!(!cache.exists());
}
