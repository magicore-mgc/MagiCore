// NPM registry client for core-web — metadata, tarball download, and quarantine checks.
// Client registry NPM cho core-web — gom network path và guard supply-chain.
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Accept header for packument fetches: abbreviated (small) normally,
/// FULL document when this client's age gate is armed (abbreviated docs
/// omit the `time` map the gate needs — fetching abbreviated under an
/// armed gate would silently keep everything).
/// (Header Accept: full doc khi cổng tuổi bật.)
fn metadata_accept_header(armed: bool) -> &'static str {
    if armed {
        "application/json"
    } else {
        "application/vnd.npm.install-v1+json"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub description: Option<String>,
    pub versions: std::collections::HashMap<String, VersionInfo>,
    #[serde(rename = "dist-tags")]
    pub dist_tags: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub time: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub version: String,
    pub dependencies: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "peerDependencies")]
    pub peer_dependencies: Option<std::collections::HashMap<String, String>>,
    #[serde(default, rename = "peerDependenciesMeta")]
    pub peer_dependencies_meta: std::collections::HashMap<String, PeerDependencyMeta>,
    #[serde(rename = "optionalDependencies")]
    pub optional_dependencies: Option<std::collections::HashMap<String, String>>,
    pub os: Option<Vec<String>>,
    pub cpu: Option<Vec<String>>,
    pub dist: Option<DistInfo>,
}

/// npm marks peer edges that must not be auto-installed via this metadata.
/// (npm dùng metadata này để đánh dấu peer không được tự cài.)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PeerDependencyMeta {
    #[serde(default)]
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistInfo {
    pub tarball: String,
    #[serde(rename = "integrity")]
    pub integrity: Option<String>,
}

pub struct NpmRegistry {
    registry_url: String,
    token: Option<String>,
    /// Multi-registry fallback chain (ITEM 1): fetch metadata từ registry kế
    /// tiếp khi primary 404/network/5xx — KHÔNG fallback khi 401/403 (auth
    /// fail = fail-closed, không leak package từ registry khác).
    fallbacks: Vec<(String, Option<String>)>,
    scoped_clients: Option<Vec<(String, ScopedRegistryClient)>>,
    /// Per-client age-gate fetch mode (P0/F6): armed by the owning
    /// provider per operation — full packuments + no stale abbreviated
    /// cache. Never a process global.
    /// (Chế độ fetch theo cổng tuổi — riêng từng client.)
    age_armed: std::sync::atomic::AtomicBool,
}

pub enum DownloadedTarball {
    Bytes(Vec<u8>),
    Streamed {
        computed_integrity: String,
        bytes_len: u64,
    },
}

fn network_profile_enabled() -> bool {
    std::env::var("MAGICORE_WEB_PROFILE_NETWORK")
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
        .map(|value| matches!(value.as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

/// 401/403 = auth fail — KHÔNG fallback sang registry khác (fail-closed).
fn is_auth_error(err: &anyhow::Error) -> bool {
    err.chain()
        .find_map(|cause| {
            cause
                .downcast_ref::<reqwest::Error>()
                .and_then(|re| re.status())
        })
        .is_some_and(|status| {
            status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN
        })
}

fn network_profile_log(kind: &str, target: &str, message: &str) {
    if network_profile_enabled() {
        eprintln!(
            "[magicore:web:network-profile] kind={} target={} {}",
            kind, target, message
        );
    }
}

/// Primary HTTP client: used for metadata fetches. Optimized for many concurrent
/// short-lived requests against the same host (registry.npmjs.org).
/// Built once and cheaply cloned (`Client` is reference-counted); a build
/// failure (broken TLS backend) is an explicit error, never a
/// process-wide panic.
fn global_http_client() -> mgc_types::MgResult<reqwest::Client> {
    use mgc_types::MgError;
    static CLIENT: std::sync::OnceLock<Result<reqwest::Client, String>> =
        std::sync::OnceLock::new();
    CLIENT
        .get_or_init(|| {
            reqwest::Client::builder()
                .redirect(registry_redirect_policy())
                // Increased pool: 256 concurrent connections per registry host.
                .pool_max_idle_per_host(256)
                .pool_idle_timeout(Duration::from_secs(90))
                .tcp_keepalive(Duration::from_secs(30))
                // Metadata responses are small; 30s is plenty.
                .timeout(Duration::from_secs(30))
                .user_agent(format!("MagiCore/{}", env!("CARGO_PKG_VERSION")))
                // H2 stream window: 4 MiB — allows multiple concurrent streams without
                // stalling when one response is slow.
                .http2_initial_stream_window_size(4 * 1024 * 1024)
                // H2 connection window: 32 MiB
                .http2_initial_connection_window_size(32 * 1024 * 1024)
                .build()
                .map_err(|error| format!("cannot build HTTP client: {error}"))
        })
        .clone()
        .map_err(MgError::Other)
}

/// Batch HTTP client: used for tarball streaming. Optimized for large bodies.
/// Same fail-closed construction as [`global_http_client`].
pub fn batch_http_client() -> mgc_types::MgResult<reqwest::Client> {
    use mgc_types::MgError;
    static BATCH_CLIENT: std::sync::OnceLock<Result<reqwest::Client, String>> =
        std::sync::OnceLock::new();
    BATCH_CLIENT
        .get_or_init(|| {
            reqwest::Client::builder()
                .redirect(registry_redirect_policy())
                .pool_max_idle_per_host(256)
                .pool_idle_timeout(Duration::from_secs(300))
                .tcp_keepalive(Duration::from_secs(60))
                // Long timeout for massive tarballs like @next/swc (>200 MB).
                .timeout(Duration::from_secs(300))
                .user_agent(format!("MagiCore/{}/batch", env!("CARGO_PKG_VERSION")))
                .http2_initial_stream_window_size(16 * 1024 * 1024)
                .http2_initial_connection_window_size(64 * 1024 * 1024)
                .build()
                .map_err(|error| format!("cannot build batch HTTP client: {error}"))
        })
        .clone()
        .map_err(MgError::Other)
}

fn jitter_ms(attempt: u32) -> u64 {
    let state = (attempt as u64)
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    (state >> 33) % 100
}

impl NpmRegistry {
    pub fn new(registry_url: &str) -> Self {
        Self {
            registry_url: registry_url.to_string(),
            token: None,
            fallbacks: Vec::new(),
            scoped_clients: None,
            age_armed: std::sync::atomic::AtomicBool::new(false),
        }
    }

    pub fn new_with_token(registry_url: &str, token: Option<String>) -> Self {
        Self {
            registry_url: registry_url.to_string(),
            token,
            fallbacks: Vec::new(),
            scoped_clients: None,
            age_armed: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Primary + fallback chain. Fallback entries: (url, token).
    pub fn new_with_chain(
        registry_url: &str,
        token: Option<String>,
        fallbacks: Vec<(String, Option<String>)>,
    ) -> Self {
        Self {
            registry_url: registry_url.to_string(),
            token,
            fallbacks,
            scoped_clients: None,
            age_armed: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Preserve typed credential scopes across every registry request.
    /// Giữ scope theo instance, không suy ngược quyền từ token hoặc URL artifact.
    pub fn new_with_scoped_chain(entries: Vec<mgc_config::chain::ScopedRegistry>) -> Result<Self> {
        let primary = entries
            .first()
            .ok_or_else(|| anyhow::anyhow!("no registry configured"))?;
        for (index, entry) in entries.iter().enumerate() {
            if entries.iter().skip(index + 1).any(|other| {
                entry.registry.url == other.registry.url
                    || (entry.credential_scope.is_some()
                        && entry.credential_scope == other.credential_scope
                        && entry.registry.token != other.registry.token)
            }) {
                return Err(anyhow::anyhow!("ambiguous registry credential chain"));
            }
        }
        let mut registry = Self::new_with_chain(
            &primary.registry.url,
            primary.registry.token.clone(),
            entries
                .iter()
                .skip(1)
                .map(|entry| (entry.registry.url.clone(), entry.registry.token.clone()))
                .collect(),
        );
        registry.scoped_clients = Some(
            entries
                .iter()
                .map(|entry| {
                    Ok((
                        entry.registry.url.clone(),
                        ScopedRegistryClient::new(entry)?,
                    ))
                })
                .collect::<Result<Vec<_>>>()?,
        );
        Ok(registry)
    }

    /// Download tasks retain immutable credential scopes and reuse connection pools.
    /// Không tái tạo từ token scalar; clone giữ policy theo instance của adapter.
    pub fn clone_for_downloads(&self) -> Self {
        Self {
            registry_url: self.registry_url.clone(),
            token: self.token.clone(),
            fallbacks: self.fallbacks.clone(),
            scoped_clients: self.scoped_clients.clone(),
            age_armed: std::sync::atomic::AtomicBool::new(self.age_gate_armed()),
        }
    }

    fn request_for(&self, target: &str, streaming: bool) -> Result<reqwest::RequestBuilder> {
        if let Some(clients) = &self.scoped_clients {
            let selected = clients
                .iter()
                .filter(|(_, client)| {
                    client.token.is_some()
                        && client
                            .scope
                            .as_ref()
                            .is_some_and(|scope| credential_path_matches(scope.as_str(), target))
                })
                .max_by_key(|(_, client)| {
                    client.scope.as_ref().map_or(0, |scope| scope.path().len())
                });
            let client = selected
                .or_else(|| clients.first())
                .ok_or_else(|| anyhow::anyhow!("no registry transport configured"))?;
            let request = client.1.get(target)?;
            return Ok(if streaming {
                request.timeout(Duration::from_secs(SCOPED_REGISTRY_TIMEOUT_SECONDS))
            } else {
                request
            });
        }
        let client = if streaming {
            batch_http_client()
        } else {
            global_http_client()
        }
        .map_err(|error| anyhow::anyhow!("{error}"))?;
        Ok(self.with_auth(client.get(target), target))
    }

    fn metadata_request_for(
        &self,
        registry_url: &str,
        token: Option<&str>,
        target: &str,
    ) -> Result<reqwest::RequestBuilder> {
        if let Some(clients) = &self.scoped_clients {
            return clients
                .iter()
                .find(|(url, _)| url == registry_url)
                .ok_or_else(|| anyhow::anyhow!("registry transport is missing"))?
                .1
                .get(target);
        }
        let request = global_http_client()
            .map_err(|error| anyhow::anyhow!("{error}"))?
            .get(target);
        Ok(if let Some(token) = token {
            request.bearer_auth(token)
        } else {
            request
        })
    }

    /// Arm/disarm full-packument fetch mode for this client (called by
    /// the owning provider when it (re-)arms its age policy).
    /// (Bật/tắt chế độ fetch full doc cho client này.)
    pub fn set_age_gate_armed(&self, armed: bool) {
        self.age_armed
            .store(armed, std::sync::atomic::Ordering::SeqCst);
    }

    /// Full packuments when armed (abbreviated docs omit `time`).
    /// (Full doc khi cổng tuổi bật.)
    pub fn age_gate_armed(&self) -> bool {
        self.age_armed.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn chain(&self) -> Vec<(String, Option<String>)> {
        let mut chain = vec![(self.registry_url.clone(), self.token.clone())];
        chain.extend(self.fallbacks.iter().cloned());
        chain
    }

    fn with_auth(&self, req: reqwest::RequestBuilder, url: &str) -> reqwest::RequestBuilder {
        if let Some(token) = &self.token
            && registry_origin_matches(&self.registry_url, url)
        {
            return req.header("Authorization", format!("Bearer {token}"));
        }
        req
    }

    pub fn auth_token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub async fn fetch_metadata(&self, package: &str) -> Result<PackageMetadata> {
        let (metadata, _) = self.fetch_metadata_with_etag(package).await?;
        Ok(metadata)
    }

    pub async fn fetch_metadata_with_etag(
        &self,
        package: &str,
    ) -> Result<(PackageMetadata, Option<String>)> {
        let chain = self.chain();
        let mut last_err: Option<anyhow::Error> = None;
        let accept = metadata_accept_header(self.age_gate_armed());

        for (url, token) in chain {
            let endpoint = format!("{}/{}", url.trim_end_matches('/'), package);
            let result = with_retry("metadata", package, || async {
                let req = self.metadata_request_for(&url, token.as_deref(), &endpoint)?;
                let resp = req
                    .header("Accept", accept)
                    .send()
                    .await?
                    .error_for_status()?;
                let etag = resp
                    .headers()
                    .get("etag")
                    .and_then(|v| v.to_str().ok())
                    .map(str::to_owned);
                let metadata: PackageMetadata = resp.json().await?;
                Ok((metadata, etag))
            })
            .await;

            // Return immediately on success — trả ngay khi thành công.
            if let Ok(ok) = result {
                return Ok(ok);
            }
            let Err(e) = result else {
                unreachable!("checked Ok arm above")
            };
            // Scoped redirect refusal is terminal, never a public fallback.
            // Lỗi ranh giới credential không được làm lộ package qua registry kế tiếp.
            let scoped_redirect_refused = self.scoped_clients.is_some()
                && e.chain().any(|cause| {
                    cause
                        .downcast_ref::<reqwest::Error>()
                        .is_some_and(|error| error.is_redirect())
                });
            if is_auth_error(&e) || scoped_redirect_refused {
                return Err(e);
            }
            last_err = Some(e);
        }

        Err(last_err
            .unwrap_or_else(|| anyhow::anyhow!("no registry configured for metadata fetch")))
    }

    pub async fn fetch_metadata_conditional(
        &self,
        package: &str,
        etag: Option<&str>,
    ) -> Result<Option<(PackageMetadata, String)>> {
        let url = format!("{}/{}", self.registry_url.trim_end_matches('/'), package);
        let etag_owned = etag.map(|s| s.to_string());
        with_retry("metadata-conditional", package, || async {
            let accept = metadata_accept_header(self.age_gate_armed());
            let mut req = self
                .metadata_request_for(&self.registry_url, self.token.as_deref(), &url)?
                .header("Accept", accept);
            if let Some(ref etag_val) = etag_owned {
                req = req.header("If-None-Match", etag_val);
            }
            let resp = req.send().await?;

            if resp.status() == reqwest::StatusCode::NOT_MODIFIED {
                return Ok(None);
            }

            let resp = resp.error_for_status()?;
            let new_etag = resp
                .headers()
                .get("etag")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_string();

            let metadata: PackageMetadata = resp.json().await?;
            Ok(Some((metadata, new_etag)))
        })
        .await
    }

    /// Classic buffered download — kept for compatibility with callers that need
    /// the raw bytes (e.g., integrity verification before store import).
    pub async fn download_tarball(&self, url: &str) -> Result<Vec<u8>> {
        with_retry("tarball", url, || async {
            let resp = self
                .request_for(url, false)?
                .send()
                .await?
                .error_for_status()?;
            let bytes = resp.bytes().await?;
            Ok(bytes.to_vec())
        })
        .await
    }

    /// **Fast-lane streaming download** — writes directly to `dest` without
    /// holding the full tarball in RAM. Returns the SHA-512 integrity hash
    /// (`sha512-<base64>`) computed on-the-fly as the stream is written.
    ///
    /// This is the primary path for large packages (> LARGE_PKG_THRESHOLD_BYTES).
    pub async fn download_tarball_to_file(
        &self,
        url: &str,
        dest: &std::path::Path,
    ) -> Result<String> {
        use futures_util::StreamExt;
        use sha2::Digest;
        use tokio::io::AsyncWriteExt;

        with_retry("tarball-stream", url, || async {
            if let Some(parent) = dest.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
            let resp = self
                .request_for(url, true)?
                .send()
                .await?
                .error_for_status()?;

            let mut file = tokio::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(dest)
                .await?;
            let mut stream = resp.bytes_stream();
            let mut hasher = sha2::Sha512::new();

            let streamed = async {
                while let Some(chunk) = stream.next().await {
                    let chunk = chunk?;
                    hasher.update(&chunk);
                    file.write_all(&chunk).await?;
                }
                file.flush().await?;
                Ok::<(), anyhow::Error>(())
            }
            .await;
            if let Err(error) = streamed {
                drop(file);
                let _ = tokio::fs::remove_file(dest).await;
                return Err(error);
            }

            let digest = hasher.finalize();
            let b64 = base64_encode(&digest);
            Ok(format!("sha512-{b64}"))
        })
        .await
    }

    pub async fn download_tarball_auto(
        &self,
        url: &str,
        dest: &std::path::Path,
    ) -> Result<DownloadedTarball> {
        use futures_util::StreamExt;
        use sha2::Digest;
        use tokio::io::AsyncWriteExt;

        with_retry("tarball-auto", url, || async {
            if let Some(parent) = dest.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }

            let resp = self
                .request_for(url, true)?
                .send()
                .await?
                .error_for_status()?;

            let content_length = resp.content_length().unwrap_or(0);
            if content_length <= LARGE_PKG_THRESHOLD_BYTES {
                let bytes = resp.bytes().await?;
                return Ok(DownloadedTarball::Bytes(bytes.to_vec()));
            }

            let mut file = tokio::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(dest)
                .await?;
            let mut stream = resp.bytes_stream();
            let mut hasher = sha2::Sha512::new();
            let mut bytes_len = 0u64;

            let streamed = async {
                while let Some(chunk) = stream.next().await {
                    let chunk = chunk?;
                    bytes_len += chunk.len() as u64;
                    hasher.update(&chunk);
                    file.write_all(&chunk).await?;
                }
                file.flush().await?;
                Ok::<(), anyhow::Error>(())
            }
            .await;
            if let Err(error) = streamed {
                drop(file);
                let _ = tokio::fs::remove_file(dest).await;
                return Err(error);
            }

            let digest = hasher.finalize();
            let b64 = base64_encode(&digest);
            Ok(DownloadedTarball::Streamed {
                computed_integrity: format!("sha512-{b64}"),
                bytes_len,
            })
        })
        .await
    }

    pub fn registry_url(&self) -> &str {
        &self.registry_url
    }
}

/// Parallel metadata fetcher — fires all requests concurrently and collects results.
/// Returns errors individually so a single bad package does not abort the batch.
pub async fn batch_fetch_metadata(
    registry_url: &str,
    packages: &[&str],
) -> Vec<(String, Result<PackageMetadata>)> {
    use futures_util::future::join_all;

    let futures: Vec<_> = packages
        .iter()
        .map(|name| {
            let reg = NpmRegistry::new(registry_url);
            let name = name.to_string();
            async move {
                let result = reg.fetch_metadata(&name).await;
                (name, result)
            }
        })
        .collect();

    join_all(futures).await
}

pub async fn batch_download_tarball(url: &str) -> Result<Vec<u8>> {
    let client = batch_http_client().map_err(|error| anyhow::anyhow!("{error}"))?;
    with_retry("batch-tarball", url, || async {
        let resp = client.get(url).send().await?.error_for_status()?;
        let bytes = resp.bytes().await?;
        Ok(bytes.to_vec())
    })
    .await
}

/// Batch download kèm auth token (registry private)
pub async fn batch_download_tarball_with_auth(url: &str, token: Option<&str>) -> Result<Vec<u8>> {
    let client = batch_http_client().map_err(|error| anyhow::anyhow!("{error}"))?;
    with_retry("batch-tarball", url, || async {
        let mut req = client.get(url);
        if let Some(t) = token {
            req = req.header("Authorization", format!("Bearer {t}"));
        }
        let resp = req.send().await?.error_for_status()?;
        let bytes = resp.bytes().await?;
        Ok(bytes.to_vec())
    })
    .await
}

/// Security guard: returns `Err` if the given package version was published within
/// the last `min_age_secs` (Supply-chain quarantine period). Returns `Ok(true)` if
/// the package passes the check, `Ok(false)` if publish time is unknown/unparseable
/// (treated as safe to avoid blocking packages without a timestamp). `0` disables.
pub fn check_publish_age(
    metadata: &PackageMetadata,
    version: &str,
    min_age_secs: i64,
) -> Result<bool, String> {
    if min_age_secs <= 0 {
        return Ok(true);
    }
    let Some(ts_str) = metadata.time.get(version) else {
        return Ok(false); // no timestamp — not quarantined
    };

    let published = chrono::DateTime::parse_from_rfc3339(ts_str)
        .map_err(|e| format!("failed to parse publish time for {}: {}", version, e))?;
    let now = chrono::Utc::now();
    let age = now.signed_duration_since(published.with_timezone(&chrono::Utc));

    if age < chrono::Duration::seconds(min_age_secs) {
        let hours = age.num_minutes() as f64 / 60.0;
        Err(format!(
            "🚨 SECURITY: Package '{}@{}' was published only {:.1}h ago (< {}h quarantine).\n   \
             This may be a supply-chain attack. Set MAGICORE_ALLOW_UNTRUSTED=1 to override.",
            metadata.name,
            version,
            hours,
            min_age_secs / 3600
        ))
    } else {
        Ok(true)
    }
}

/// Threshold in bytes above which `download_tarball_to_file` is used instead of
/// buffered `download_tarball`. Set to 2 MiB.
pub const LARGE_PKG_THRESHOLD_BYTES: u64 = 2 * 1024 * 1024;

fn base64_encode(bytes: &[u8]) -> String {
    // Manual base64 to avoid new dep — use alphabet from RFC 4648.
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((bytes.len() * 4).div_ceil(3));
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = if chunk.len() > 1 {
            chunk[1] as usize
        } else {
            0
        };
        let b2 = if chunk.len() > 2 {
            chunk[2] as usize
        } else {
            0
        };
        out.push(ALPHABET[b0 >> 2] as char);
        out.push(ALPHABET[((b0 & 0x3) << 4) | (b1 >> 4)] as char);
        if chunk.len() > 1 {
            out.push(ALPHABET[((b1 & 0xF) << 2) | (b2 >> 6)] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(ALPHABET[b2 & 0x3F] as char);
        } else {
            out.push('=');
        }
    }
    out
}

async fn with_retry<F, Fut, T>(kind: &str, target: &str, mut f: F) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut last_error = None;
    for attempt in 0..4u32 {
        match f().await {
            Ok(value) => {
                if attempt > 0 {
                    network_profile_log(
                        kind,
                        target,
                        &format!("success_after_retry attempt={}", attempt + 1),
                    );
                }
                return Ok(value);
            }
            Err(err) => {
                network_profile_log(
                    kind,
                    target,
                    &format!("retry attempt={} error={}", attempt + 1, err),
                );
                last_error = Some(err);
                if attempt < 3 {
                    let base_ms = 50u64 * (2u64.pow(attempt));
                    let jitter = jitter_ms(attempt);
                    tokio::time::sleep(Duration::from_millis(base_ms + jitter)).await;
                }
            }
        }
    }
    // The loop above always runs at least once and returns on the first
    // success, so a fall-through guarantees an error was captured. The
    // `None` arm is unreachable by construction — it carries a real error
    // anyway so a future refactor can never turn it into a panic.
    match last_error {
        Some(error) => Err(error),
        None => Err(anyhow::anyhow!(
            "retry loop for {kind} {target} exhausted attempts without capturing an error"
        )),
    }
}

#[cfg(test)]
#[path = "test/npm_registry_tests.rs"]
mod tests;

// Explicit registry credentials are restricted to the same HTTP origin.
// Token của registry không được gửi sang scheme/host/port khác.
fn registry_origin_matches(registry_url: &str, request_url: &str) -> bool {
    let (Ok(registry), Ok(request)) = (
        reqwest::Url::parse(registry_url),
        reqwest::Url::parse(request_url),
    ) else {
        return false;
    };
    matches!(registry.scheme(), "http" | "https")
        && matches!(request.scheme(), "http" | "https")
        && registry.username().is_empty()
        && request.username().is_empty()
        && registry.password().is_none()
        && request.password().is_none()
        && registry.host_str().is_some()
        && registry.scheme() == request.scheme()
        && registry.host_str() == request.host_str()
        && registry.port_or_known_default() == request.port_or_known_default()
}

// Keep bounded redirects without crossing protocol credential boundaries.
// Giữ redirect mặc định có giới hạn, nhưng không chuyển token qua scheme khác.
fn registry_redirect_policy() -> reqwest::redirect::Policy {
    reqwest::redirect::Policy::custom(|attempt| {
        if let Some(previous) = attempt.previous().last()
            && previous.scheme() != attempt.url().scheme()
        {
            return attempt.error(std::io::Error::other(
                "registry redirect changes URL scheme; configure the final secure URL explicitly",
            ));
        }
        reqwest::redirect::Policy::default().redirect(attempt)
    })
}

/// Build a request without separating the selected token from its path scope.
/// Native callers phải truyền cả scoped registry; artifact public ngoài scope không có token.
pub fn scoped_registry_request(
    registry: &mgc_config::chain::ScopedRegistry,
    target: &str,
) -> Result<reqwest::RequestBuilder> {
    ScopedRegistryClient::new(registry)?.get(target)
}

/// An instance-owned transport keeps one credential client and its immutable scope.
/// Không cache token toàn process; adapter giữ instance để tái dùng connection pool.
#[derive(Clone)]
pub struct ScopedRegistryClient {
    scope: Option<reqwest::Url>,
    token: Option<String>,
    credential_client: Option<reqwest::Client>,
    public_client: reqwest::Client,
}

impl ScopedRegistryClient {
    /// Build clients once; missing or malformed credential scope fails explicitly.
    /// Credential có token thì phải có scope hợp lệ trước khi tạo transport.
    pub fn new(registry: &mgc_config::chain::ScopedRegistry) -> Result<Self> {
        let scope = registry.credential_scope.clone();
        let token = registry.registry.token.clone();
        let credential_client = if token.is_some() {
            let selected = scope
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("credential scope is missing"))?;
            if !credential_path_matches(selected.as_str(), selected.as_str()) {
                return Err(anyhow::anyhow!("invalid registry credential scope"));
            }
            let selected = selected.to_string();
            Some(
                reqwest::Client::builder()
                    .redirect(reqwest::redirect::Policy::custom(move |attempt| {
                        if !credential_path_matches(&selected, attempt.url().as_str()) {
                            return attempt.error(std::io::Error::other(
                                "credential redirect leaves authorized registry scope",
                            ));
                        }
                        reqwest::redirect::Policy::default().redirect(attempt)
                    }))
                    .timeout(Duration::from_secs(SCOPED_REGISTRY_TIMEOUT_SECONDS))
                    .user_agent(format!("MagiCore/{}", env!("CARGO_PKG_VERSION")))
                    .build()?,
            )
        } else {
            None
        };
        Ok(Self {
            scope,
            token,
            credential_client,
            public_client: global_http_client().map_err(|error| anyhow::anyhow!("{error}"))?,
        })
    }

    /// Reuse the appropriate client; public requests never inherit credential headers.
    /// Mỗi request kiểm scope lại, không đặt token vào default headers của client.
    pub fn get(&self, target: &str) -> Result<reqwest::RequestBuilder> {
        let parsed = reqwest::Url::parse(target)
            .map_err(|_| anyhow::anyhow!("invalid registry request URL"))?;
        if !matches!(parsed.scheme(), "http" | "https")
            || parsed.host_str().is_none()
            || !parsed.username().is_empty()
            || parsed.password().is_some()
        {
            return Err(anyhow::anyhow!(
                "registry request URL must use HTTP without userinfo"
            ));
        }
        let authorized = self.token.is_some()
            && self
                .scope
                .as_ref()
                .is_some_and(|scope| credential_path_matches(scope.as_str(), target));
        if authorized {
            let client = self
                .credential_client
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("credential transport is missing"))?;
            let token = self
                .token
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("credential token is missing"))?;
            Ok(client.get(parsed).bearer_auth(token))
        } else {
            Ok(self.public_client.get(parsed))
        }
    }
}

const SCOPED_REGISTRY_TIMEOUT_SECONDS: u64 = 300;
const MAX_CREDENTIAL_PATH_DECODE_LAYERS: usize = 8;

// Match origin and a path segment boundary, refusing ambiguous encoded traversal.
// Giữ hỗ trợ @scope%2fpkg; dot segment hoặc backslash không được cấp token.
fn credential_path_matches(scope: &str, target: &str) -> bool {
    if !registry_origin_matches(scope, target) {
        return false;
    }
    let (Ok(scope), Ok(target)) = (reqwest::Url::parse(scope), reqwest::Url::parse(target)) else {
        return false;
    };
    if !credential_path_is_unambiguous(scope.path())
        || !credential_path_is_unambiguous(target.path())
    {
        return false;
    }
    let prefix = scope.path().trim_end_matches('/');
    target.path() == prefix || target.path().starts_with(&format!("{prefix}/"))
}

// Bounded structural decoding is a policy check, not a server normalization claim.
// Không khẳng định mô hình mọi server; mã hóa quá sâu hoặc lỗi thì không gửi token.
fn credential_path_is_unambiguous(path: &str) -> bool {
    let mut bytes = path.as_bytes().to_vec();
    for _ in 0..MAX_CREDENTIAL_PATH_DECODE_LAYERS {
        if bytes.contains(&b'\\')
            || bytes
                .split(|byte| *byte == b'/')
                .any(|segment| segment == b"." || segment == b"..")
        {
            return false;
        }
        let mut decoded = Vec::with_capacity(bytes.len());
        let mut index = 0;
        while index < bytes.len() {
            if bytes[index] == b'%' {
                if index + 2 >= bytes.len() {
                    return false;
                }
                let (Some(high), Some(low)) = (
                    (bytes[index + 1] as char).to_digit(16),
                    (bytes[index + 2] as char).to_digit(16),
                ) else {
                    return false;
                };
                decoded.push((high * 16 + low) as u8);
                index += 3;
            } else {
                decoded.push(bytes[index]);
                index += 1;
            }
        }
        if decoded == bytes {
            return true;
        }
        bytes = decoded;
    }
    false
}
