//! (Lệnh template: publish/fetch kernel-làm registry — Bun/pnpm create thành mgc-create-*, Q13)
//! publish: pack templates/{core}/{name} thành tarball mgc-create-<core>-<name>
//! fetch: tải tarball → ~/.mgc/templates/{core}/{name} (cache; resolve sẽ thấy Disk)
use anyhow::{Result, bail};
use clap::Parser;
use mgc_config::npmrc::NpmRc;
use mgc_publish::auth::resolve_auth;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile;

const DEFAULT_REGISTRY: &str = "https://registry.npmjs.org/";
const TEMPLATE_FETCH_TIMEOUT_SECS: u64 = 30;
const MAX_TEMPLATE_REDIRECTS: usize = 5;
const MAX_TEMPLATE_METADATA_BYTES: usize = 16 * 1024 * 1024;
const MAX_TEMPLATE_TARBALL_BYTES: usize = 64 * 1024 * 1024;
const MAX_TEMPLATE_VERSION_BYTES: u64 = 1024;
/// Tên package trên registry: `mgc-create-<core>-<name>` (Bun/pnpm naming, plan r3, T5).
/// Name chứa dấu / (layer rel như backend/go/gin) được dash-hoá;
/// nếu name đã bắt đầu "<core>/" thì core không lặp (publish-all truyền rel đầy đủ).
pub fn registry_package_name(core: &str, name: &str) -> String {
    let prefix = format!("{core}/");
    let inner = name.strip_prefix(&prefix).unwrap_or(name);
    format!("mgc-create-{core}-{}", inner.replace('/', "-"))
}

/// Cache dir: MGC_DATA_DIR/templates (override qua MGC_TEMPLATES_DIR).
pub fn templates_cache_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("MGC_TEMPLATES_DIR") {
        return PathBuf::from(dir);
    }
    mgc_platform::paths::mgc_data_dir().join("templates")
}

/// Serialize embedded-layer refreshes across MagiCore processes. Without this
/// lock, concurrent first-run processes can each remove/extract the same
/// shared cache directory and observe a partially materialized layer.
/// (Tuần tự hóa refresh layer embedded giữa các process MagiCore.)
pub(crate) fn lock_embedded_cache_layer(cache_root: &Path, rel: &str) -> Result<fs::File> {
    let lock_dir = cache_root.join(".locks");
    fs::create_dir_all(&lock_dir)?;
    let lock_dir_metadata = fs::symlink_metadata(&lock_dir)?;
    if template_cache_is_link(&lock_dir_metadata) || !lock_dir_metadata.is_dir() {
        bail!("embedded template cache lock directory is not a real directory");
    }
    let key = blake3::hash(rel.as_bytes()).to_hex().to_string();
    let lock_path = lock_dir.join(format!("embedded-{key}.lock"));
    let mut options = fs::OpenOptions::new();
    options.read(true).write(true).create(true).truncate(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW);
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        // FILE_FLAG_OPEN_REPARSE_POINT: open the lock object, not its target.
        // (Mở chính lock, không đi theo reparse point trên Windows.)
        options.custom_flags(0x0020_0000);
    }
    let file = options.open(&lock_path)?;
    let metadata = fs::symlink_metadata(&lock_path)?;
    if !metadata.is_file() || template_cache_is_link(&metadata) || !file.metadata()?.is_file() {
        bail!("embedded template cache lock is not a regular file");
    }
    fs2::FileExt::lock_exclusive(&file)?;
    Ok(file)
}

/// Rel path của layer trong templates/ — name flat (react-vite → web/frontend/react-vite)
/// hoặc full rel (backend/go/gin → giữ nguyên).
/// Cache cũng nằm theo rel này để TemplateRoot::resolve thấy.
pub fn layer_rel(core: &str, name: &str) -> String {
    if name.starts_with(&format!("{core}/")) {
        return name.to_string();
    }
    if name.contains('/') {
        return format!("{core}/{name}");
    }
    if core == "web" {
        format!("web/frontend/{name}")
    } else {
        format!("{core}/{name}")
    }
}

/// Đường dẫn cache của layer — flat (v1: chỉ giữ latest; version pin là P2).
pub fn cached_layer_path(core: &str, name: &str) -> PathBuf {
    templates_cache_dir().join(layer_rel(core, name))
}

/// `mgc template` subcommands
#[derive(Debug, Parser, Clone)]
pub enum TemplateCmd {
    /// Fetch template layer từ registry về ~/.mgc/templates (create sẽ tự thấy)
    #[command(about = "Fetch template layer from registry into cache ~/.mgc/templates")]
    Fetch(TemplateFetchArgs),
    /// Publish kernel template thành package mgc-create-<core>-<name>
    #[command(about = "Publish kernel template layer to registry")]
    Publish(TemplatePublishArgs),
    /// Publish mọi layer web có template.toml+sources (registry-first source of truth)
    #[command(about = "Publish every template layer under templates/web to registry")]
    PublishAll,
}

/// `mgc template fetch <core> <name> [--registry URL] [--tag latest]`
#[derive(Debug, Parser, Clone)]
pub struct TemplateFetchArgs {
    pub core: String,
    pub name: String,
    #[arg(long, help = "override registry URL")]
    pub registry: Option<String>,
    #[arg(long, help = "dist-tag (default: latest)")]
    pub tag: Option<String>,
}

/// `mgc template publish <core> <name> [--registry URL] [--version]`
#[derive(Debug, Parser, Clone)]
pub struct TemplatePublishArgs {
    pub core: String,
    pub name: String,
    #[arg(long, help = "override registry URL")]
    pub registry: Option<String>,
    #[arg(long, help = "package version (default: 0.1.0)")]
    pub version: Option<String>,
}

/// Dispatch mgc template subcommands.
pub async fn run(cmd: TemplateCmd) -> Result<()> {
    match cmd {
        TemplateCmd::Fetch(args) => {
            fetch(args).await?;
        }
        TemplateCmd::Publish(args) => {
            publish(args).await?;
        }
        TemplateCmd::PublishAll => {
            publish_all().await?;
        }
    }
    Ok(())
}

/// Publish toàn bộ layer có đủ template.toml+sources, mọi core (web/lib/cicd/...).
/// Nguồn đọc qua TemplateRoot::resolve (MAGICORE_TEMPLATE_DIR → registry cache).
async fn publish_all() -> Result<()> {
    let root = crate::scaffold::template_root::TemplateRoot::resolve("");
    let root_path = root.path();
    if !root_path.is_dir() {
        return Err(crate::error::web_templates_missing(root_path));
    }
    let mut layers: Vec<PathBuf> = Vec::new();
    collect_layers_under(root_path, &mut layers);
    if layers.is_empty() {
        return Err(crate::error::no_web_layer_found(root_path));
    }
    for layer in &layers {
        let rel = layer
            .strip_prefix(root_path)
            .map_err(|_| crate::error::web_template_path_missing(&root_path.display().to_string()))?
            .to_string_lossy()
            .replace('\\', "/");
        // rel = "web/frontend/react-vite" → core = segment đầu (web/lib/cicd).
        let core = rel.split('/').next().unwrap_or("web").to_string();
        let pkg = registry_package_name(&core, &rel);
        println!("  publishing {} (as {pkg})", rel);
        publish(TemplatePublishArgs {
            core,
            name: rel,
            registry: None,
            version: None,
        })
        .await?;
    }
    println!("Publish-all done: {} layers across all cores", layers.len());
    Ok(())
}

/// Thu thập mọi folder có template.toml + sources dưới root (không đệ quy vào leaf đã chứa).
fn collect_layers_under(root: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path.join("template.toml").is_file() && path.join("sources").is_dir() {
                out.push(path);
            } else {
                collect_layers_under(&path, out);
            }
        }
    }
}

/// Publish kernel template layer `templates/{core}/{name}` thành package
/// `mgc-create-<core>-<name>` trên registry (mgc-pack tarball + npm PUT).
async fn publish(args: TemplatePublishArgs) -> Result<()> {
    validate_template_identity(&args.core, &args.name)?;
    let rel = layer_rel(&args.core, &args.name);
    let layer = crate::scaffold::template_root::TemplateRoot::resolve(&rel)
        .path()
        .to_path_buf();
    if !template_layer_has_contract(&layer)? {
        return Err(crate::error::template_layer_missing_manifest(&rel));
    }

    let version = args.version.unwrap_or_else(|| "0.1.0".to_string());
    semver::Version::parse(&version).map_err(|_| crate::error::template_metadata_invalid())?;
    let name = registry_package_name(&args.core, &args.name);
    let registry = template_registry_url(&select_registry(args.registry.as_deref())?)?.to_string();

    let temp_dir = tempfile::tempdir()?;
    let tarball_path = temp_dir
        .path()
        .join(format!("{}.tgz", name.replace('/', "-")));
    let pack_result = mgc_pack::tarball::pack(&layer, &tarball_path, &format!("{name}-{version}"))?;

    let npmrc = NpmRc::load(Path::new(".")).unwrap_or_default();
    let auth = resolve_auth(&npmrc, &registry, None, None).unwrap_or_default();
    if auth.is_empty() {
        let host = url::Url::parse(&registry)
            .map(|u| u.host_str().unwrap_or_default().to_string())
            .unwrap_or_default();
        if host.contains("npmjs") || host.contains("npm") {
            return Err(crate::error::template_publish_auth_required());
        }
    }

    upload_tarball(&registry, &auth, &name, &version, &tarball_path).await?;

    let manifest = serde_json::json!({
        "name": name,
        "version": version,
        "description": format!("MagiCore kernel template {}/{}", args.core, args.name),
        "main": "template.toml",
    });
    put_manifest(&registry, &auth, &name, &version, &manifest, &pack_result).await?;
    println!(
        "Template '{}' published → {}/{}/{}",
        name,
        registry.trim_end_matches('/'),
        name,
        version
    );
    Ok(())
}

/// Fetch template layer về cache registry.
pub async fn fetch(args: TemplateFetchArgs) -> Result<PathBuf> {
    let (path, _) = fetch_layer_at(args, &templates_cache_dir(), None).await?;
    Ok(path)
}

async fn fetch_layer_at(
    args: TemplateFetchArgs,
    cache_root: &Path,
    auth_override: Option<&mgc_publish::auth::Auth>,
) -> Result<(PathBuf, String)> {
    validate_template_identity(&args.core, &args.name)?;
    let registry = select_registry(args.registry.as_deref())?;
    let name = registry_package_name(&args.core, &args.name);
    let tag = args.tag.unwrap_or_else(|| "latest".to_string());

    let registry_url = template_registry_url(&registry)?;
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(TEMPLATE_FETCH_TIMEOUT_SECS))
        .build()
        .map_err(|_| crate::error::template_request_failed())?;
    let auto = match auth_override {
        Some(auth) => auth.clone(),
        None => register_fetch_auth(&registry).await,
    };
    let authorization = auto.header_value();
    let mut metadata_url = registry_url.clone();
    metadata_url
        .path_segments_mut()
        .map_err(|_| crate::error::template_download_url_invalid())?
        .pop_if_empty()
        .push(&name);
    let metadata = download_template_bytes(
        &client,
        metadata_url,
        &registry_url,
        authorization.as_deref(),
        MAX_TEMPLATE_METADATA_BYTES,
    )
    .await?;
    let meta: serde_json::Value =
        serde_json::from_slice(&metadata).map_err(|_| crate::error::template_metadata_invalid())?;

    let versions = meta
        .get("versions")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| crate::error::registry_missing_field("versions"))?;
    let version = versions
        .get(&tag)
        .map(|_| tag.as_str())
        .or_else(|| {
            meta.get("dist-tags")
                .and_then(|tags| tags.get(&tag))
                .and_then(serde_json::Value::as_str)
        })
        .ok_or_else(|| crate::error::registry_missing_field(&format!("dist-tags/{tag}")))?;
    semver::Version::parse(version).map_err(|_| crate::error::template_metadata_invalid())?;
    let dist = versions
        .get(version)
        .and_then(|entry| entry.get("dist"))
        .ok_or_else(|| crate::error::registry_missing_field("version.dist"))?;
    let tarball = dist
        .get("tarball")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| crate::error::registry_missing_field("dist.tarball"))?;
    let integrity = dist
        .get("integrity")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| crate::error::registry_missing_field("dist.integrity"))?;
    let tarball_url = registry_url
        .join(tarball)
        .map_err(|_| crate::error::template_download_url_invalid())?;
    let bytes = download_template_bytes(
        &client,
        tarball_url,
        &registry_url,
        authorization.as_deref(),
        MAX_TEMPLATE_TARBALL_BYTES,
    )
    .await?;
    verify_template_integrity(&bytes, integrity)?;

    fs::create_dir_all(cache_root)?;
    if template_cache_is_link(&fs::symlink_metadata(cache_root)?) {
        return Err(crate::error::template_cache_unsafe_path(cache_root));
    }
    let cache_root = cache_root.canonicalize()?;
    let rel = layer_rel(&args.core, &args.name);
    let target = cache_root.join(&rel);
    let _cache_lock = lock_embedded_cache_layer(&cache_root, &rel)?;
    extract_tarball_into(&bytes, &target, Some(version))?;

    println!("Template '{}' fetched → {}", name, target.display());
    Ok((target, version.to_string()))
}

fn template_registry_url(value: &str) -> Result<url::Url> {
    let mut url =
        url::Url::parse(value).map_err(|_| crate::error::template_download_url_invalid())?;
    validate_template_download_url(&url, &url)?;
    if url.query().is_some() {
        return Err(crate::error::template_download_url_invalid());
    }
    url.path_segments_mut()
        .map_err(|_| crate::error::template_download_url_invalid())?
        .pop_if_empty()
        .push("");
    Ok(url)
}

fn validate_template_download_url(url: &url::Url, registry: &url::Url) -> Result<()> {
    let loopback = url.host_str().is_some_and(|host| {
        host.eq_ignore_ascii_case("localhost")
            || host
                .trim_start_matches('[')
                .trim_end_matches(']')
                .parse::<std::net::IpAddr>()
                .is_ok_and(|address| address.is_loopback())
    });
    if !matches!(url.scheme(), "https" | "http")
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
        || (url.scheme() == "http" && (!loopback || registry.scheme() == "https"))
    {
        return Err(crate::error::template_download_url_invalid());
    }
    Ok(())
}

fn template_auth_allowed(registry: &url::Url, target: &url::Url) -> bool {
    if registry.origin() != target.origin() {
        return false;
    }
    let prefix = registry.path().trim_end_matches('/');
    if prefix.is_empty() {
        return true;
    }
    let path = target.path();
    let lower = path.to_ascii_lowercase();
    !["%2f", "%5c", "%2e", "%25"]
        .iter()
        .any(|encoded| lower.contains(encoded))
        && (path == prefix || path.starts_with(&format!("{prefix}/")))
}

/// Re-evaluate credential scope on every hop; never let redirects forward auth implicitly.
/// Từng redirect phải xét lại origin/path; CDN ngoài registry không nhận credential.
async fn download_template_bytes(
    client: &reqwest::Client,
    mut target: url::Url,
    registry: &url::Url,
    authorization: Option<&str>,
    limit: usize,
) -> Result<Vec<u8>> {
    let started = std::time::Instant::now();
    let budget = std::time::Duration::from_secs(TEMPLATE_FETCH_TIMEOUT_SECS);
    for redirect in 0..=MAX_TEMPLATE_REDIRECTS {
        validate_template_download_url(&target, registry)?;
        let remaining = budget.saturating_sub(started.elapsed());
        if remaining.is_zero() {
            return Err(crate::error::template_request_failed());
        }
        let mut request = client.get(target.clone()).timeout(remaining);
        if template_auth_allowed(registry, &target)
            && let Some(header) = authorization
        {
            request = request.header(reqwest::header::AUTHORIZATION, header);
        }
        let mut response = request.send().await.map_err(|error| {
            if error.is_connect() || error.is_timeout() {
                crate::error::template_transport_unavailable()
            } else {
                crate::error::template_request_failed()
            }
        })?;
        if response.status().is_redirection() {
            if redirect == MAX_TEMPLATE_REDIRECTS {
                return Err(crate::error::template_redirect_invalid());
            }
            let location = response
                .headers()
                .get(reqwest::header::LOCATION)
                .and_then(|value| value.to_str().ok())
                .ok_or_else(crate::error::template_redirect_invalid)?;
            target = target
                .join(location)
                .map_err(|_| crate::error::template_redirect_invalid())?;
            continue;
        }
        if !response.status().is_success() {
            return Err(crate::error::template_download_status(
                response.status().as_u16(),
            ));
        }
        if response
            .content_length()
            .is_some_and(|length| length > limit as u64)
        {
            return Err(crate::error::template_download_too_large(limit));
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|_| crate::error::template_request_failed())?
        {
            if chunk.len() > limit.saturating_sub(bytes.len()) {
                return Err(crate::error::template_download_too_large(limit));
            }
            bytes.extend_from_slice(&chunk);
        }
        return Ok(bytes);
    }
    Err(crate::error::template_redirect_invalid())
}

fn verify_template_integrity(bytes: &[u8], integrity: &str) -> Result<()> {
    use base64::Engine;
    use sha2::{Digest, Sha256, Sha384, Sha512};
    let rank = |token: &str| {
        let algorithm = token
            .split_once('-')
            .map(|(algorithm, _)| algorithm)
            .unwrap_or("");
        if algorithm.eq_ignore_ascii_case("sha512") {
            3
        } else if algorithm.eq_ignore_ascii_case("sha384") {
            2
        } else if algorithm.eq_ignore_ascii_case("sha256") {
            1
        } else {
            0
        }
    };
    let strongest = integrity
        .split_whitespace()
        .map(rank)
        .max()
        .filter(|rank| *rank > 0)
        .ok_or_else(crate::error::template_integrity_invalid)?;
    let actual = match strongest {
        3 => Sha512::digest(bytes).to_vec(),
        2 => Sha384::digest(bytes).to_vec(),
        _ => Sha256::digest(bytes).to_vec(),
    };
    let mut saw_valid_digest = false;
    for token in integrity
        .split_whitespace()
        .filter(|token| rank(token) == strongest)
    {
        let hash = token
            .split_once('-')
            .map(|(_, hash)| hash)
            .unwrap_or("")
            .split('?')
            .next()
            .unwrap_or("");
        if hash.len() > actual.len().div_ceil(3) * 4 {
            continue;
        }
        let Ok(expected) = base64::engine::general_purpose::STANDARD
            .decode(hash)
            .or_else(|_| base64::engine::general_purpose::STANDARD_NO_PAD.decode(hash))
        else {
            continue;
        };
        if expected.len() != actual.len() {
            continue;
        }
        saw_valid_digest = true;
        if expected == actual {
            return Ok(());
        }
    }
    if !saw_valid_digest {
        return Err(crate::error::template_integrity_invalid());
    }
    Err(crate::error::template_integrity_mismatch())
}

/// Đảm bảo template layer có sẵn (disk / cache) — nếu thiếu và registry có config,
/// tự fetch. Registry-first: klass pnpm đua `create-*` từ registry.
///
/// Returns typed ScaffoldResolveStatus thay vì bool - KHÔNG được bỏ qua!
pub async fn ensure_layer(
    rel: &str,
) -> Result<
    crate::scaffold::resolver::ScaffoldResolveStatus,
    crate::scaffold::resolver::ScaffoldResolveError,
> {
    ensure_layer_in(rel, &templates_cache_dir(), None, None).await
}

async fn ensure_layer_in(
    requested_layer: &str,
    configured_cache_root: &Path,
    registry_override: Option<&str>,
    auth_override: Option<&mgc_publish::auth::Auth>,
) -> Result<
    crate::scaffold::resolver::ScaffoldResolveStatus,
    crate::scaffold::resolver::ScaffoldResolveError,
> {
    use crate::scaffold::embedded::EmbeddedKernel;
    use crate::scaffold::resolver::{ScaffoldResolveError, ScaffoldResolveStatus};
    use crate::scaffold::spec::{CoreKind, ScaffoldRef, parse_scaffold_spec};

    let (prefix, leaf) = requested_layer.rsplit_once('/').ok_or_else(|| {
        ScaffoldResolveError::Other(crate::error::template_layer_identity_invalid().to_string())
    })?;
    let core_str = prefix.split('/').next().unwrap_or("");
    let core =
        CoreKind::from_str_core(core_str).ok_or_else(|| ScaffoldResolveError::UnsupportedCore {
            core: core_str.to_string(),
        })?;
    let spec = parse_scaffold_spec(core, leaf)
        .map_err(|error| ScaffoldResolveError::Other(error.to_string()))?;
    let canonical_layer = format!("{prefix}/{}", spec.normalized_name);
    validate_template_identity(core_str, &canonical_layer)
        .map_err(|error| ScaffoldResolveError::Other(error.to_string()))?;
    let rel = canonical_layer.as_str();
    let name_segment = spec.normalized_name.as_str();
    fs::create_dir_all(configured_cache_root)
        .map_err(|error| ScaffoldResolveError::Other(error.to_string()))?;
    if template_cache_is_link(
        &fs::symlink_metadata(configured_cache_root)
            .map_err(|error| ScaffoldResolveError::Other(error.to_string()))?,
    ) {
        return Err(ScaffoldResolveError::Other(
            crate::error::template_cache_unsafe_path(configured_cache_root).to_string(),
        ));
    }
    let cache_root = configured_cache_root
        .canonicalize()
        .map_err(|error| ScaffoldResolveError::Other(error.to_string()))?;
    let dev_candidate = std::env::var_os("MAGICORE_TEMPLATE_DIR")
        .map(|root| PathBuf::from(root).join(rel))
        .filter(|path| path.is_dir());
    let is_dev_override = dev_candidate.is_some();
    let visible_path = dev_candidate.unwrap_or_else(|| cache_root.join(rel));
    // Inspect existing ancestors before reading cache markers or manifests.
    // Không đọc xuyên symlink cha; cache chưa tồn tại không cần tạo thư mục rỗng.
    if !is_dev_override {
        let mut ancestor = visible_path.parent();
        while let Some(path) = ancestor {
            if path == cache_root {
                break;
            }
            match fs::symlink_metadata(path) {
                Ok(metadata) if template_cache_is_link(&metadata) || !metadata.is_dir() => {
                    return Err(ScaffoldResolveError::Other(
                        crate::error::template_cache_unsafe_path(path).to_string(),
                    ));
                }
                Ok(_) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(ScaffoldResolveError::Other(error.to_string())),
            }
            ancestor = path.parent();
        }
    }
    let has_contract = template_layer_has_contract(&visible_path)
        .map_err(|error| ScaffoldResolveError::Other(error.to_string()))?;
    let embedded_version = EmbeddedKernel::layer_version_path(rel)
        .or_else(|| EmbeddedKernel::layer_version(core_str, name_segment));
    let cached_version = read_template_cache_version(&visible_path)
        .map_err(|error| ScaffoldResolveError::Other(error.to_string()))?;
    if has_contract
        && (is_dev_override
            || (template_cache_matches_ref(&spec.requested_ref, cached_version.as_deref())
                && (cached_version.is_some() || embedded_version.is_none())))
    {
        if !is_dev_override {
            ensure_template_cache_parent(visible_path.parent().unwrap_or(&cache_root))
                .map_err(|error| ScaffoldResolveError::Other(error.to_string()))?;
        }
        if cached_version.is_none() && !is_dev_override {
            mgc_ui::warning(
                "Using an unversioned local template cache; fetch the layer again to record its version",
            );
        }
        return Ok(ScaffoldResolveStatus::CacheHit {
            layer: rel.to_string(),
            version: cached_version,
            path: visible_path,
        });
    }
    if is_dev_override && !has_contract {
        return Err(ScaffoldResolveError::Other(
            crate::error::template_layer_missing_manifest(rel).to_string(),
        ));
    }
    // A compiled processor needs no extracted snapshot or unused per-layer cache.
    // Chỉ mặc định dùng processor builtin; version/tag tường minh vẫn phải resolve registry.
    if matches!(&spec.requested_ref, ScaffoldRef::Default)
        && !has_contract
        && crate::scaffold::processor::has_builtin_scaffold_processor(rel)
    {
        return Ok(ScaffoldResolveStatus::Embedded {
            layer: rel.to_string(),
        });
    }
    let use_embedded = match &spec.requested_ref {
        ScaffoldRef::Default => true,
        ScaffoldRef::Version(version) => embedded_version == Some(version.as_str()),
        _ => false,
    };

    /// Legacy embedded-cache freshness: a `.mgc-embedded-version` marker
    /// records which embedded version was extracted. Missing marker (caches
    /// from before versioning) or version mismatch → re-extract, so template
    /// updates are never shadowed by stale caches.
    /// (Tươi cache embedded: marker version, lệch thì giải nén lại.)
    fn embedded_cache_fresh(cache_target: &std::path::Path, version: Option<&str>) -> bool {
        let marker = cache_target.join(".mgc-embedded-version");
        let cached = std::fs::read_to_string(&marker)
            .ok()
            .map(|v| v.trim().to_string());
        match (cached, version) {
            (Some(cached), Some(version)) => {
                cached == version && template_layer_has_contract(cache_target).unwrap_or(false)
            }
            _ => false,
        }
    }

    /// Record the extracted embedded version in the cache (best-effort —
    /// a missing marker just re-extracts next time).
    /// (Ghi marker version embedded vào cache.)
    fn stamp_embedded_cache(
        cache_target: &std::path::Path,
        version: Option<&str>,
    ) -> anyhow::Result<()> {
        if let Some(version) = version {
            std::fs::create_dir_all(cache_target)?;
            std::fs::write(cache_target.join(".mgc-embedded-version"), version)?;
        }
        Ok(())
    }

    /// Remove only a stale real directory. A symlink or non-directory cache
    /// entry is an error; cleanup failures must not be followed by overlaying
    /// new embedded files and stamping the result as fresh.
    /// (Chỉ xóa thư mục cache cũ thật; lỗi dọn phải dừng, không ghi đè chồng.)
    fn remove_stale_embedded_cache(cache_target: &std::path::Path) -> anyhow::Result<()> {
        match std::fs::symlink_metadata(cache_target) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error.into()),
            Ok(metadata) if template_cache_is_link(&metadata) || !metadata.is_dir() => {
                anyhow::bail!(
                    "refusing to replace non-directory or linked embedded cache '{}'",
                    cache_target.display()
                )
            }
            Ok(_) => {
                std::fs::remove_dir_all(cache_target)?;
                Ok(())
            }
        }
    }
    // 1. Check embedded kernel first
    // Try full path first (web/shared/base), then short form (web/vanilla)
    if use_embedded && EmbeddedKernel::has_layer_path(rel) {
        // Hold a per-layer OS lock across freshness check, replacement, and
        // version stamping. The lock file lives outside the directory being
        // refreshed, so another process cannot unlink the lock inode.
        // (Giữ OS lock xuyên kiểm tra, thay cache và ghi version marker.)
        let _cache_lock = lock_embedded_cache_layer(&cache_root, rel).map_err(|error| {
            ScaffoldResolveError::Other(format!("Failed to lock embedded template cache: {error}"))
        })?;
        // Extract embedded to legacy cache location so processor can find it.
        let cache_target = cache_root.join(rel);
        let version = EmbeddedKernel::layer_version_path(rel);
        if !embedded_cache_fresh(&cache_target, version) {
            remove_stale_embedded_cache(&cache_target).map_err(|error| {
                ScaffoldResolveError::Other(format!(
                    "Failed to replace stale embedded template cache: {error}"
                ))
            })?;
            EmbeddedKernel::extract_layer_path(rel, &cache_target).map_err(|e| {
                ScaffoldResolveError::Other(format!("Failed to extract embedded kernel: {}", e))
            })?;
            stamp_embedded_cache(&cache_target, version).map_err(|error| {
                ScaffoldResolveError::Other(format!(
                    "Failed to stamp embedded template cache: {error}"
                ))
            })?;
        }
        return Ok(ScaffoldResolveStatus::Embedded {
            layer: rel.to_string(),
        });
    }

    let base_name = name_segment.split('@').next().unwrap_or(name_segment);
    if use_embedded && EmbeddedKernel::has_layer(core_str, base_name) {
        // Short-form embedded entries use the same cross-process protection.
        // (Layer embedded dạng rút gọn dùng cùng khóa liên-process.)
        let _cache_lock = lock_embedded_cache_layer(&cache_root, rel).map_err(|error| {
            ScaffoldResolveError::Other(format!("Failed to lock embedded template cache: {error}"))
        })?;
        let cache_target = cache_root.join(rel);
        let version = EmbeddedKernel::layer_version(core_str, base_name);
        if !embedded_cache_fresh(&cache_target, version) {
            remove_stale_embedded_cache(&cache_target).map_err(|error| {
                ScaffoldResolveError::Other(format!(
                    "Failed to replace stale embedded template cache: {error}"
                ))
            })?;
            EmbeddedKernel::extract_layer(core_str, base_name, &cache_target).map_err(|e| {
                ScaffoldResolveError::Other(format!("Failed to extract embedded kernel: {}", e))
            })?;
            stamp_embedded_cache(&cache_target, version).map_err(|error| {
                ScaffoldResolveError::Other(format!(
                    "Failed to stamp embedded template cache: {error}"
                ))
            })?;
        }
        return Ok(ScaffoldResolveStatus::Embedded {
            layer: rel.to_string(),
        });
    }

    // Use the same npm protocol, configured registry and security boundary as explicit fetch.
    // Tự động tải qua đúng protocol/config/guard của template fetch, vào cache processor đang đọc.
    let tag = match &spec.requested_ref {
        ScaffoldRef::Version(version) | ScaffoldRef::DistTag(version) => version.clone(),
        ScaffoldRef::Default => "latest".to_string(),
        ScaffoldRef::Range(_) => {
            return Err(ScaffoldResolveError::Other(
                crate::error::template_layer_identity_invalid().to_string(),
            ));
        }
    };
    match fetch_layer_at(
        TemplateFetchArgs {
            core: core_str.to_string(),
            name: rel.to_string(),
            registry: registry_override.map(str::to_string),
            tag: Some(tag.clone()),
        },
        &cache_root,
        auth_override,
    )
    .await
    {
        Ok((path, version)) => Ok(ScaffoldResolveStatus::Fetched {
            layer: rel.to_string(),
            version,
            path,
        }),
        Err(error) => Err(template_resolution_error(
            error,
            rel,
            core_str,
            &spec.normalized_name,
            &tag,
        )),
    }
}

fn template_cache_matches_ref(
    requested: &crate::scaffold::spec::ScaffoldRef,
    cached: Option<&str>,
) -> bool {
    use crate::scaffold::spec::ScaffoldRef;
    match requested {
        ScaffoldRef::Default => true,
        ScaffoldRef::Version(version) => cached == Some(version.as_str()),
        ScaffoldRef::DistTag(_) | ScaffoldRef::Range(_) => false,
    }
}

fn template_resolution_error(
    error: anyhow::Error,
    layer: &str,
    core: &str,
    name: &str,
    tag: &str,
) -> crate::scaffold::resolver::ScaffoldResolveError {
    use crate::scaffold::resolver::ScaffoldResolveError;
    if error.is::<crate::error::TemplateTransportUnavailable>() {
        return ScaffoldResolveError::RegistryUnavailable {
            registry: "configured template registry".to_string(),
            offline: false,
        };
    }
    if let Some(status) = error.downcast_ref::<crate::error::TemplateHttpStatus>() {
        if matches!(status.0, 404 | 410) {
            return ScaffoldResolveError::RequiredLayerMissing {
                layer: layer.to_string(),
                core: core.to_string(),
                template: name.to_string(),
                tag: tag.to_string(),
                attempted_sources: error.to_string(),
            };
        }
        if status.0 == 429 || status.0 >= 500 {
            return ScaffoldResolveError::RegistryUnavailable {
                registry: format!("configured template registry (HTTP {})", status.0),
                offline: false,
            };
        }
    }
    ScaffoldResolveError::Other(error.to_string())
}

pub(crate) fn template_layer_has_contract(path: &Path) -> Result<bool> {
    for (entry, directory) in [
        (path.to_path_buf(), true),
        (path.join("template.toml"), false),
        (path.join("sources"), true),
    ] {
        let metadata = match fs::symlink_metadata(&entry) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
            Err(error) => return Err(error.into()),
        };
        if template_cache_is_link(&metadata)
            || (directory && !metadata.is_dir())
            || (!directory && !metadata.is_file())
        {
            return Err(crate::error::template_cache_unsafe_path(&entry));
        }
    }
    crate::scaffold::processor::validate_template_manifest(path)?;
    Ok(true)
}

pub(crate) fn read_template_cache_version(path: &Path) -> Result<Option<String>> {
    let marker = path.join(".mgc-version");
    let metadata = match fs::symlink_metadata(&marker) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    if template_cache_is_link(&metadata)
        || !metadata.is_file()
        || metadata.len() > MAX_TEMPLATE_VERSION_BYTES
    {
        return Err(crate::error::template_cache_unsafe_path(&marker));
    }
    let version = fs::read_to_string(&marker)?.trim().to_string();
    semver::Version::parse(&version).map_err(|_| crate::error::template_metadata_invalid())?;
    Ok(Some(version))
}

/// Extract tarball entries vào target, bỏ segment đầu của entry name
/// (npm tarball prefix `mgc-create-.../...`).
pub(crate) fn extract_tarball_into(
    bytes: &[u8],
    target: &Path,
    version: Option<&str>,
) -> Result<()> {
    let parent = target
        .parent()
        .ok_or_else(|| crate::error::template_cache_unsafe_path(target))?;
    ensure_template_cache_parent(parent)?;
    let previous = match fs::symlink_metadata(target) {
        Ok(metadata) if template_cache_is_link(&metadata) || !metadata.is_dir() => {
            return Err(crate::error::template_cache_unsafe_path(target));
        }
        Ok(_) => true,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
        Err(error) => return Err(error.into()),
    };

    // Validate the complete archive in a private same-filesystem staging tree.
    // Giữ cache đang dùng nguyên vẹn cho tới khi toàn archive đã qua extractor an toàn.
    let staging = tempfile::tempdir_in(parent)?;
    let unpacked = staging.path().join("unpacked");
    mgc_resolver::protocols::archive::extract_tar_gz(bytes, &unpacked)?;
    let entries = fs::read_dir(&unpacked)?.collect::<std::io::Result<Vec<_>>>()?;
    if entries.len() != 1 || !entries[0].file_type()?.is_dir() {
        return Err(crate::error::template_archive_layout_invalid());
    }
    let packaged = entries[0].path();
    if !packaged.join("template.toml").is_file() || !packaged.join("sources").is_dir() {
        return Err(crate::error::template_archive_layout_invalid());
    }
    crate::scaffold::processor::validate_template_manifest(&packaged)?;
    if let Some(version) = version {
        semver::Version::parse(version).map_err(|_| crate::error::template_metadata_invalid())?;
        fs::write(packaged.join(".mgc-version"), version)?;
    }

    let backup = staging.path().join("previous");
    if previous {
        fs::rename(target, &backup)?;
    }
    if let Err(error) = fs::rename(&packaged, target) {
        if previous {
            // Preserve the backup if rollback itself fails; never delete the old cache.
            // Nếu rollback lỗi, giữ backup trên đĩa thay vì để TempDir xóa cache cũ.
            if let Err(rollback) = fs::rename(&backup, target) {
                let retained = staging.keep();
                return Err(crate::error::template_cache_rollback_failed(
                    target, &retained, &error, &rollback,
                ));
            }
        }
        return Err(error.into());
    }
    staging.close()?;
    Ok(())
}

fn ensure_template_cache_parent(parent: &Path) -> Result<()> {
    use std::path::Component;
    let mut current = PathBuf::new();
    for component in parent.components() {
        match component {
            Component::Prefix(_) | Component::RootDir => {
                current.push(component.as_os_str());
                continue;
            }
            Component::CurDir => continue,
            Component::ParentDir => return Err(crate::error::template_cache_unsafe_path(parent)),
            Component::Normal(part) => current.push(part),
        }
        match fs::symlink_metadata(&current) {
            Ok(metadata) if template_cache_is_link(&metadata) || !metadata.is_dir() => {
                return Err(crate::error::template_cache_unsafe_path(&current));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => fs::create_dir(&current)?,
            Err(error) => return Err(error.into()),
        }
    }
    Ok(())
}

fn validate_template_identity(core: &str, name: &str) -> Result<()> {
    if crate::scaffold::spec::CoreKind::from_str_core(core).is_none()
        || name.is_empty()
        || name.contains(['\\', ':'])
        || name.chars().any(char::is_control)
        || name
            .split('/')
            .any(|part| part.is_empty() || matches!(part, "." | ".."))
    {
        return Err(crate::error::template_layer_identity_invalid());
    }
    Ok(())
}

/// Reject links and Windows junction/reparse objects before cache mutation.
/// Chặn cả junction/reparse Windows, không chỉ symlink thông thường.
pub(crate) fn template_cache_is_link(metadata: &fs::Metadata) -> bool {
    if metadata.file_type().is_symlink() {
        return true;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0400;
        metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
    }
    #[cfg(not(windows))]
    false
}

#[cfg(test)]
#[path = "test/template.rs"]
mod tests;

/// Resolve registry: --registry flag → MGC_NPM_REGISTRY env → .npmrc → default.
fn select_registry(flag: Option<&str>) -> Result<String> {
    if let Some(url) = flag {
        return Ok(url.to_string());
    }
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Ok(url) = std::env::var("MGC_NPM_REGISTRY")
        && !url.is_empty()
    {
        return Ok(url);
    }
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Ok(npmrc) = NpmRc::load(Path::new("."))
        && let Some(url) = npmrc.registry_for(None)
    {
        return Ok(url);
    }
    Ok(DEFAULT_REGISTRY.to_string())
}

/// Auth resolution cho fetch — registry công khai không cần; registry private
/// (token admin) yêu cầu header, nên lấy auth nếu cấu hình (npmrc/env).
async fn register_fetch_auth(registry: &str) -> mgc_publish::auth::Auth {
    let npmrc = NpmRc::load(Path::new("."));
    let npmrc = match npmrc {
        Ok(n) => n,
        Err(_) => return mgc_publish::auth::Auth::default(),
    };
    resolve_auth(&npmrc, registry, None, None)
        .unwrap_or_else(|_| mgc_publish::auth::Auth::default())
}

/// Construct upload URLs from encoded path segments and a validated registry base.
/// Không nối chuỗi URL từ tên/version; giữ registry prefix và không cho query injection.
fn template_publish_url(
    registry: &url::Url,
    name: &str,
    version: Option<&str>,
) -> Result<url::Url> {
    let mut target = registry.clone();
    let mut path = target
        .path_segments_mut()
        .map_err(|_| crate::error::template_download_url_invalid())?;
    path.pop_if_empty().push(name);
    if let Some(version) = version {
        semver::Version::parse(version).map_err(|_| crate::error::template_metadata_invalid())?;
        let unscoped = name.rsplit('/').next().unwrap_or(name);
        path.push("-").push(&format!("{unscoped}-{version}.tgz"));
    }
    drop(path);
    Ok(target)
}

/// Publishing never follows redirects with a write body or registry credentials.
/// PUT chỉ tới registry đã validate; redirect bị từ chối, không đọc/log error body.
async fn put_template_bytes(
    registry: &url::Url,
    target: url::Url,
    auth: &mgc_publish::auth::Auth,
    body: Vec<u8>,
    json: bool,
) -> Result<()> {
    validate_template_download_url(&target, registry)?;
    if !template_auth_allowed(registry, &target) {
        return Err(crate::error::template_download_url_invalid());
    }
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(TEMPLATE_FETCH_TIMEOUT_SECS))
        .build()
        .map_err(|_| crate::error::template_request_failed())?;
    let mut request = client.put(target).body(body);
    if json {
        request = request.header(reqwest::header::CONTENT_TYPE, "application/json");
    }
    if let Some(header) = auth.header_value() {
        request = request.header(reqwest::header::AUTHORIZATION, header);
    }
    let response = request
        .send()
        .await
        .map_err(|_| crate::error::template_request_failed())?;
    if !response.status().is_success() {
        return Err(crate::error::template_publish_status(
            response.status().as_u16(),
        ));
    }
    Ok(())
}

/// PUT tarball blob to the selected registry only.
/// Tải archive lên đúng registry, giới hạn lượng dữ liệu đọc vào RAM.
async fn upload_tarball(
    registry_url: &str,
    auth: &mgc_publish::auth::Auth,
    name: &str,
    version: &str,
    tarball_path: &Path,
) -> Result<()> {
    use std::io::Read;
    let registry = template_registry_url(registry_url)?;
    let target = template_publish_url(&registry, name, Some(version))?;
    let mut data = Vec::new();
    fs::File::open(tarball_path)?
        .take(MAX_TEMPLATE_TARBALL_BYTES as u64 + 1)
        .read_to_end(&mut data)?;
    if data.len() > MAX_TEMPLATE_TARBALL_BYTES {
        return Err(crate::error::template_download_too_large(
            MAX_TEMPLATE_TARBALL_BYTES,
        ));
    }
    put_template_bytes(&registry, target, auth, data, false).await
}

/// PUT manifest metadata (dist-tags + versions).
/// Metadata dùng cùng origin/path policy với archive và không echo body registry.
async fn put_manifest(
    registry_url: &str,
    auth: &mgc_publish::auth::Auth,
    name: &str,
    version: &str,
    manifest: &serde_json::Value,
    pack_result: &mgc_pack::tarball::PackResult,
) -> Result<()> {
    let registry = template_registry_url(registry_url)?;
    let target = template_publish_url(&registry, name, None)?;
    let tarball = template_publish_url(&registry, name, Some(version))?;
    let mut body = serde_json::Map::new();
    body.insert("_id".into(), serde_json::Value::String(name.into()));
    body.insert("name".into(), serde_json::Value::String(name.into()));
    body.insert("dist-tags".into(), serde_json::json!({"latest": version}));
    body.insert("maintainers".into(), serde_json::json!([]));
    body.insert(
        "time".into(),
        serde_json::json!({"created": "", "modified": ""}),
    );
    body.insert("private".into(), serde_json::json!(false));
    let mut versions = serde_json::Map::new();
    let mut version_obj = manifest.clone();
    version_obj
        .as_object_mut()
        .ok_or_else(crate::error::template_metadata_invalid)?
        .insert(
            "dist".into(),
            serde_json::json!({
                "tarball": tarball.as_str(),
                "shasum": pack_result.shasum,
                "integrity": pack_result.integrity,
                "fileCount": pack_result.entry_count,
                "unpackedSize": pack_result.unpacked_size,
            }),
        );
    versions.insert(version.into(), version_obj);
    body.insert("versions".into(), serde_json::Value::Object(versions));
    let bytes = serde_json::to_vec(&body).map_err(|_| crate::error::template_metadata_invalid())?;
    if bytes.len() > MAX_TEMPLATE_METADATA_BYTES {
        return Err(crate::error::template_download_too_large(
            MAX_TEMPLATE_METADATA_BYTES,
        ));
    }
    put_template_bytes(&registry, target, auth, bytes, true).await
}
