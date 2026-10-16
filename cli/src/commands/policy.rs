//! Signed exception-manifest loading — nạp manifest exception có ký.
//!
//! Rotation/distribution channel, file-based (Spec rev4 §5/§16.7 — decided):
//! explicit `MGC_POLICY_MANIFEST` file first, then the project-local
//! `.mgc/policy/exceptions.json`, then the shared data-dir cache. Trust
//! roots come from the explicit user environment variable
//! `MGC_POLICY_TRUST_KEYS` (BLAKE3 hex ids), never from project-controlled
//! `mgc.toml`. Newest valid `issued_at` wins; revocations union across
//! verified snapshots.
//! (Kênh rotation/phân phối, dạng file (Spec rev4 §16.7 — đã chốt): file
//! `MGC_POLICY_MANIFEST` tường minh trước, rồi `.mgc/policy/exceptions.json`
//! của project, rồi cache data-dir dùng chung. Trust root từ
//! `MGC_POLICY_TRUST_KEYS` do user cấu hình, không lấy từ `mgc.toml`.
//! Bản hợp lệ `issued_at` mới nhất thắng; thu hồi hợp nhất qua snapshot đã
//! xác minh.)

use std::collections::BTreeSet;
use std::io::Read;
use std::path::{Path, PathBuf};

const MAX_POLICY_MANIFEST_BYTES: usize = 1024 * 1024;

// The executor consults manifests only with an active lock-bound profile.
// Missing or invalid inputs keep native-deny in force.
// Executor chỉ tra manifest cùng profile hiệu lực gắn lock. Thiếu hoặc sai
// input thì native-deny tiếp tục được áp dụng.
/// Environment override pointing at one manifest file.
/// Env override chỉ tới một file manifest.
#[allow(dead_code)]
pub const POLICY_MANIFEST_ENV: &str = "MGC_POLICY_MANIFEST";

/// User-controlled trust roots for signed policy manifests (comma-separated
/// BLAKE3 key IDs). Project `mgc.toml` cannot authorize its own exceptions.
/// Trust root do người dùng cấu hình (ID key BLAKE3, phân tách bằng dấu phẩy).
/// `mgc.toml` của project không thể tự cấp quyền exception cho chính nó.
pub const POLICY_TRUST_KEYS_ENV: &str = "MGC_POLICY_TRUST_KEYS";

/// Collect candidate manifest documents without failing the caller on
/// unreadable files: a corrupt optional policy file warns and is skipped —
/// the base native-deny layers stay enforced regardless.
/// (Gom tài liệu manifest ứng viên mà không fail caller khi file lỗi đọc:
/// file policy tùy nghi hỏng thì cảnh báo và bỏ qua — các lớp native-deny
/// gốc vẫn thực thi dù thế nào.)
#[allow(dead_code)]
fn read_candidates(project_root: &Path) -> Vec<mgc_exec::policy_manifest::PolicyManifest> {
    let mut paths: Vec<PathBuf> = Vec::new();
    if let Some(raw) = std::env::var_os(POLICY_MANIFEST_ENV).filter(|v| !v.is_empty()) {
        paths.push(PathBuf::from(raw));
    }
    paths.push(
        project_root
            .join(".magicore")
            .join(mgc_exec::policy_manifest::cache_rel_path()),
    );
    paths.push(
        mgc_platform::paths::mgc_data_dir().join(mgc_exec::policy_manifest::cache_rel_path()),
    );

    let mut manifests = Vec::new();
    for path in paths {
        let mut bytes = Vec::new();
        let read_result = std::fs::File::open(&path).and_then(|file| {
            file.take((MAX_POLICY_MANIFEST_BYTES + 1) as u64)
                .read_to_end(&mut bytes)
        });
        match read_result {
            Ok(size) if size <= MAX_POLICY_MANIFEST_BYTES => {}
            Ok(_) => {
                mgc_ui::warning(&format!(
                    "ignoring oversized policy manifest '{}': maximum size is 1 MiB",
                    path.display()
                ));
                continue;
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => {
                mgc_ui::warning(&format!(
                    "ignoring unreadable policy manifest '{}': {e}",
                    path.display()
                ));
                continue;
            }
        }
        match serde_json::from_slice(&bytes) {
            Ok(manifest) => manifests.push(manifest),
            Err(e) => {
                mgc_ui::warning(&format!(
                    "ignoring malformed policy manifest '{}': {e}",
                    path.display()
                ));
            }
        }
    }
    manifests
}

/// Load the signed policy, lane context and lock-bound profile for one
/// operation. Exceptions require all three to match.
/// (Nạp policy ký, ngữ cảnh lane và profile gắn với lock. Exception cần cả
/// ba cùng khớp.)
pub fn load_policy_for_run(
    project_root: &Path,
    operation: &str,
) -> (
    Option<mgc_exec::policy_manifest::PolicyManifest>,
    Option<mgc_exec::policy::PolicyContext>,
    Option<mgc_exec::policy::LaneProfile>,
) {
    let manifest = load_active_exception_manifest(project_root);
    let context = lane_context(project_root, operation);
    let profile = context
        .as_ref()
        .and_then(|context| active_profile(project_root, context));
    (manifest, context, profile)
}

/// Resolve a lane context from locked web framework metadata first, then
/// from project configuration. An unknown version remains empty and cannot
/// match a qualified profile or exception.
/// (Ưu tiên framework web đã khóa, rồi dùng cấu hình project. Version chưa
/// biết để rỗng nên không khớp profile hay exception đã qualify.)
fn lane_context(project_root: &Path, operation: &str) -> Option<mgc_exec::policy::PolicyContext> {
    let config = mgc_config::project::ProjectConfig::load(project_root)
        .ok()
        .flatten()?;
    let (framework, version) = match crate::commands::freshness::resolve_web_framework(project_root)
    {
        Ok(Some((framework, version))) => (framework, version),
        Ok(None) => (
            config
                .frameworks
                .first()
                .cloned()
                .unwrap_or_else(|| config.ecosystem.clone()),
            String::new(),
        ),
        Err(_) => return None,
    };
    Some(mgc_exec::policy::PolicyContext {
        framework,
        version,
        os: std::env::consts::OS.to_string(),
        operation: operation.to_string(),
    })
}

/// Load only a profile bound to the exact lane tuple and current lock bytes.
/// Invalid or stale profile data cannot enable a manifest exception.
/// Chỉ nạp profile gắn đúng lane tuple và bytes lock hiện tại. Profile sai
/// hoặc hết hạn không thể mở manifest exception.
fn active_profile(
    project_root: &Path,
    context: &mgc_exec::policy::PolicyContext,
) -> Option<mgc_exec::policy::LaneProfile> {
    let lock_bytes = std::fs::read(project_root.join("mgc.lock")).ok()?;
    let digest = mgc_crypto::blake3_signer::Blake3Hasher::hash_bytes(&lock_bytes).to_hex();
    match mgc_exec::spawn_profile::find_active_profile(
        &mgc_platform::paths::mgc_data_dir().join("profiles"),
        &context.framework,
        &context.version,
        &context.os,
        &context.operation,
        Some(&digest),
    ) {
        Ok(profile) => profile,
        Err(error) => {
            mgc_ui::warning(&format!(
                "ignoring invalid spawn profile for policy exception lookup: {error:?}"
            ));
            None
        }
    }
}

/// Environment variable selecting URL distribution for manifests.
/// Env chọn phân phối manifest qua URL.
pub const POLICY_MANIFEST_URL_ENV: &str = "MGC_POLICY_MANIFEST_URL";

/// Environment variable selecting OCI-registry distribution
/// (`oci://host/repo:tag`, default tag `latest`).
/// Env chọn phân phối qua OCI registry.
pub const POLICY_MANIFEST_OCI_ENV: &str = "MGC_POLICY_MANIFEST_OCI";

/// Optional bearer token for a private OCI registry.
/// Token Bearer cho OCI registry private.
pub const POLICY_OCI_TOKEN_ENV: &str = "MGC_POLICY_OCI_TOKEN";

/// Media type of the OCI config blob carrying the policy document.
/// Media type của config blob OCI chứa tài liệu policy.
pub const POLICY_OCI_CONFIG_MEDIA_TYPE: &str = "application/vnd.magicore.policy.exceptions.v2+json";

/// URL distribution refresh when configured (Spec rev4 §16.7 — decided):
/// fetch → verify → atomically refresh the data-dir cache, then file
/// loading sees it. Failures warn and fall back to the file cache — an
/// unreachable policy server never fails the command.
/// (Refresh phân phối URL khi được cấu hình (Spec rev4 §16.7 — đã chốt):
/// tải → xác minh → refresh atomically cache data-dir, rồi nạp file sẽ thấy.
/// Lỗi thì cảnh báo và dùng cache file — server policy unreachable không
/// bao giờ fail lệnh.)
pub async fn refresh_url_manifest_if_configured(_project_root: &Path) {
    let url = std::env::var_os(POLICY_MANIFEST_URL_ENV)
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string_lossy().into_owned());
    let oci = std::env::var_os(POLICY_MANIFEST_OCI_ENV)
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string_lossy().into_owned());
    if url.is_none() && oci.is_none() {
        return;
    }
    let trusted = trusted_policy_key_ids();
    if trusted.is_empty() {
        mgc_ui::warning(
            "policy manifest distribution is configured but MGC_POLICY_TRUST_KEYS is empty; skipping refresh",
        );
        return;
    }
    let cache =
        mgc_platform::paths::mgc_data_dir().join(mgc_exec::policy_manifest::cache_rel_path());
    if let Some(url) = url {
        if oci.is_some() {
            mgc_ui::warning(
                "both policy manifest distribution variables are set; trying HTTPS URL first and OCI only if it fails",
            );
        }
        match refresh_manifest_from_url(&url, &trusted, &cache).await {
            Ok(_) => return,
            Err(e) => mgc_ui::warning(&format!(
                "policy manifest URL refresh failed ({e:#}); continuing with the file cache"
            )),
        }
    }
    if let Some(reference) = oci {
        // Registry channel (Spec rev4 follow-up — decided): OCI artifacts
        // carry the policy document; it can serve as fallback for a failed
        // URL refresh or as the only configured source.
        // (Kênh registry (follow-up Spec rev4 — đã chốt): artifact OCI mang
        // tài liệu policy; dùng fallback khi URL lỗi hoặc làm nguồn duy nhất.)
        if let Err(e) = refresh_manifest_from_oci(&reference, &trusted, &cache).await {
            mgc_ui::warning(&format!(
                "policy OCI refresh failed ({e:#}); continuing with the file cache"
            ));
        }
    }
}

/// Parse `oci://host[/prefix]/repo[:tag]` into base URL, repo and tag.
/// Tag defaults to `latest`. Plain HTTP registries are refused except
/// loopback (same rule as URL distribution).
/// (Parse `oci://host[/prefix]/repo[:tag]` thành base URL, repo và tag.
/// Tag mặc định `latest`. Registry HTTP thường bị từ chối trừ loopback
/// (cùng luật phân phối URL).)
pub fn parse_oci_reference(reference: &str) -> Option<(String, String, String)> {
    let rest = reference.strip_prefix("oci://")?;
    let slash = rest.find('/')?;
    let (host, path) = (&rest[..slash], &rest[slash + 1..]);
    if host.is_empty() || path.is_empty() {
        return None;
    }
    let is_loopback = matches!(host, "localhost" | "127.0.0.1" | "::1")
        || host.starts_with("127.")
        || host.starts_with("localhost:");
    let base_url = if is_loopback {
        format!("http://{host}")
    } else {
        format!("https://{host}")
    };
    let (repo, tag) = match path.rfind(':') {
        Some(i) if !path[i + 1..].contains('/') => (&path[..i], path[i + 1..].to_string()),
        _ => (path, "latest".to_string()),
    };
    if repo.is_empty() || tag.is_empty() {
        return None;
    }
    Some((base_url, repo.to_string(), tag))
}

/// Verify fetched bytes and atomically replace the cache (shared by URL and
/// OCI channels — one verification path, no per-channel drift).
/// (Xác minh bytes tải về và thay atomically cache (dùng chung cho kênh URL
/// và OCI — một đường xác minh, không lệch theo kênh).)
fn verify_and_cache(
    bytes: &[u8],
    trusted_hex_ids: &[String],
    cache_path: &Path,
) -> anyhow::Result<mgc_exec::policy_manifest::PolicyManifest> {
    const MAX_MANIFEST_BYTES: usize = 1024 * 1024;
    if bytes.len() > MAX_MANIFEST_BYTES {
        anyhow::bail!("policy manifest exceeds 1 MiB; download was not cached");
    }
    let manifest: mgc_exec::policy_manifest::PolicyManifest = serde_json::from_slice(bytes)
        .map_err(|e| anyhow::anyhow!("policy manifest is not valid JSON: {e}"))?;
    let now = mgc_exec::policy_manifest::now_unix_secs();
    let selected = mgc_exec::policy_manifest::select_active_manifest(
        vec![manifest],
        trusted_hex_ids,
        &[],
        now,
    )
    .ok_or_else(|| anyhow::anyhow!("fetched policy manifest does not verify"))?;
    if let Some(parent) = cache_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    // Serialize the complete read/compare/write transaction across MGC
    // processes. (Tuần tự hóa trọn giao dịch đọc/so sánh/ghi giữa các process.)
    let _cache_lock = lock_policy_manifest_cache(cache_path)?;
    if let Some(previous) = read_verified_cached_manifest(cache_path, trusted_hex_ids) {
        if selected.issued_at < previous.issued_at {
            return Err(crate::error::policy_manifest_update_rejected(
                "incoming snapshot is older than the verified cache",
            ));
        }
        if selected.issued_at == previous.issued_at {
            if mgc_exec::policy_manifest::canonical_bytes(&selected)
                == mgc_exec::policy_manifest::canonical_bytes(&previous)
            {
                return Ok(previous);
            }
            return Err(crate::error::policy_manifest_update_rejected(
                "different snapshots share the cached issued_at timestamp",
            ));
        }

        let previous_revoked = previous
            .revoked_keys
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let incoming_revoked = selected
            .revoked_keys
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if let Some(removed_key) = previous_revoked.difference(&incoming_revoked).next() {
            return Err(crate::error::policy_manifest_update_rejected(&format!(
                "newer snapshot dropped previously revoked key '{removed_key}'"
            )));
        }
    }
    let tmp = cache_path.with_extension("json.tmp");
    std::fs::write(&tmp, serde_json::to_vec_pretty(&selected)?)?;
    std::fs::rename(&tmp, cache_path)?;
    Ok(selected)
}

/// OS-backed lock for the shared policy cache; the lock file is retained so
/// concurrent writers always lock the same inode.
/// (Khóa OS cho cache policy dùng chung; giữ file để writer cùng khóa một inode.)
fn lock_policy_manifest_cache(cache_path: &Path) -> anyhow::Result<std::fs::File> {
    use std::fs::OpenOptions;

    let lock_path = cache_path.with_extension("lock");
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true).truncate(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW);
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        options.custom_flags(0x0020_0000);
    }
    let file = options.open(&lock_path).map_err(|error| {
        crate::error::policy_manifest_cache_lock_failed(&format!(
            "cannot open '{}': {error}",
            lock_path.display()
        ))
    })?;
    let metadata = std::fs::symlink_metadata(&lock_path).map_err(|error| {
        crate::error::policy_manifest_cache_lock_failed(&format!(
            "cannot inspect '{}': {error}",
            lock_path.display()
        ))
    })?;
    let open_metadata = file.metadata().map_err(|error| {
        crate::error::policy_manifest_cache_lock_failed(&format!(
            "cannot inspect opened '{}': {error}",
            lock_path.display()
        ))
    })?;
    if !metadata.is_file() || metadata.file_type().is_symlink() || !open_metadata.is_file() {
        return Err(crate::error::policy_manifest_cache_lock_failed(
            "lock path is not a regular file",
        ));
    }
    fs2::FileExt::lock_exclusive(&file).map_err(|error| {
        crate::error::policy_manifest_cache_lock_failed(&format!(
            "cannot acquire '{}': {error}",
            lock_path.display()
        ))
    })?;
    Ok(file)
}

/// Read a trusted cached snapshot even after its exception payload expires.
/// Its signature and revocation history still constrain future cache updates.
/// (Đọc snapshot cache đã tin cậy kể cả khi payload exception hết hạn. Chữ ký
/// và lịch sử thu hồi vẫn ràng buộc các lần cập nhật cache sau.)
fn read_verified_cached_manifest(
    cache_path: &Path,
    trusted_hex_ids: &[String],
) -> Option<mgc_exec::policy_manifest::PolicyManifest> {
    let file = std::fs::File::open(cache_path).ok()?;
    let mut bytes = Vec::new();
    let size = file
        .take((MAX_POLICY_MANIFEST_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .ok()?;
    if size > MAX_POLICY_MANIFEST_BYTES {
        return None;
    }
    let manifest: mgc_exec::policy_manifest::PolicyManifest =
        serde_json::from_slice(&bytes).ok()?;
    let signer_hex = mgc_exec::policy_manifest::key_id_hex(&manifest.key_id)?;
    if !trusted_hex_ids.contains(&signer_hex) {
        return None;
    }
    let verification_time = manifest.expires_at.saturating_sub(1);
    mgc_exec::policy_manifest::verify_manifest(
        &manifest,
        std::slice::from_ref(&manifest.key_id),
        &[],
        verification_time,
    )
    .ok()?;
    Some(manifest)
}

/// Explicit manifest distribution over HTTPS (Spec rev4 §16.7 — decided):
/// fetch → verify → atomically replace the data-dir cache. Plain HTTP is
/// refused except loopback; bodies over 1 MiB are refused before caching.
/// (Phân phối manifest tường minh qua HTTPS (Spec rev4 §16.7 — đã chốt):
/// tải → xác minh → thay atomically cache data-dir. HTTP thường bị từ chối
/// trừ loopback; body quá 1 MiB bị từ chối trước khi cache.)
pub async fn refresh_manifest_from_url(
    url: &str,
    trusted_hex_ids: &[String],
    cache_path: &Path,
) -> anyhow::Result<mgc_exec::policy_manifest::PolicyManifest> {
    let parsed: reqwest::Url = url
        .parse()
        .map_err(|e| anyhow::anyhow!("policy manifest URL invalid: {e}"))?;
    let is_https = parsed.scheme() == "https";
    let is_loopback = matches!(
        parsed.host_str(),
        Some("localhost") | Some("127.0.0.1") | Some("::1")
    );
    if !is_https && !is_loopback {
        anyhow::bail!("policy manifest URL must use HTTPS unless it targets loopback");
    }
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    let bytes = client.get(url).send().await?.bytes().await?;
    verify_and_cache(&bytes, trusted_hex_ids, cache_path)
}

/// OCI-registry distribution (Spec rev4 follow-up — decided): the policy
/// document travels as an OCI config blob so lanes can pin it by digest or
/// tag without a new registry protocol. Layers must be empty — anything
/// else is refused (no smuggled payloads).
/// (Phân phối qua OCI registry (follow-up Spec rev4 — đã chốt): tài liệu
/// policy đi trong config blob OCI để lane ghim theo digest/tag mà không cần
/// protocol registry mới. Layers phải rỗng — còn lại bị từ chối (không payload
/// lén).)
pub async fn refresh_manifest_from_oci(
    reference: &str,
    trusted_hex_ids: &[String],
    cache_path: &Path,
) -> anyhow::Result<mgc_exec::policy_manifest::PolicyManifest> {
    let (base_url, repo, tag) = parse_oci_reference(reference)
        .ok_or_else(|| anyhow::anyhow!("policy OCI reference invalid: '{reference}'"))?;
    let mut client = mgc_oci::client::OciClient::new(base_url, None)?;
    if let Some(token) = std::env::var_os(POLICY_OCI_TOKEN_ENV).filter(|v| !v.is_empty()) {
        client = client.with_token(token.to_string_lossy().into_owned());
    }
    let manifest = client.pull_manifest(&repo, &tag).await?;
    if manifest.config.media_type != POLICY_OCI_CONFIG_MEDIA_TYPE {
        anyhow::bail!(
            "OCI artifact is not a policy manifest (config media type '{}')",
            manifest.config.media_type
        );
    }
    if !manifest.layers.is_empty() {
        anyhow::bail!("policy OCI artifact must not carry layers");
    }
    let bytes = client.pull_blob(&repo, &manifest.config.digest).await?;
    verify_and_cache(&bytes, trusted_hex_ids, cache_path)
}

/// Load the active exception manifest for a project, if any verifies.
/// Returns `None` when no candidate verifies — callers keep native-deny.
/// (Nạp manifest exception hiệu lực cho project, nếu có bản nào xác minh
/// được. Trả `None` khi không bản nào xác minh được — caller giữ native-deny.)
pub fn load_active_exception_manifest(
    project_root: &Path,
) -> Option<mgc_exec::policy_manifest::PolicyManifest> {
    let trusted = trusted_policy_key_ids();
    if trusted.is_empty() {
        return None;
    }
    let now = mgc_exec::policy_manifest::now_unix_secs();
    mgc_exec::policy_manifest::select_active_manifest(
        read_candidates(project_root),
        &trusted,
        &[],
        now,
    )
}

/// Parse the explicit user-level key list. Invalid entries are discarded;
/// an empty result leaves all signed exceptions disabled.
/// Parse danh sách key do user cấu hình. Bỏ entry sai; kết quả rỗng thì
/// mọi signed exception vẫn đóng.
fn trusted_policy_key_ids() -> Vec<String> {
    std::env::var(POLICY_TRUST_KEYS_ENV)
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|key| key.len() == 16 && key.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .map(str::to_ascii_lowercase)
        .collect()
}

#[cfg(test)]
mod cache_lock_tests {
    use super::lock_policy_manifest_cache;
    use fs2::FileExt;

    #[test]
    fn policy_cache_lock_serializes_independent_file_handles() {
        let dir = tempfile::tempdir().unwrap();
        let cache_path = dir.path().join("exceptions.json");
        let guard = lock_policy_manifest_cache(&cache_path).unwrap();
        let contender = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(cache_path.with_extension("lock"))
            .unwrap();

        assert!(contender.try_lock_exclusive().is_err());
        drop(guard);
        assert!(contender.try_lock_exclusive().is_ok());
    }
}
