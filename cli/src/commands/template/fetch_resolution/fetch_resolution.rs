use super::error;
use crate::commands::template::{
    MAX_TEMPLATE_METADATA_BYTES, MAX_TEMPLATE_TARBALL_BYTES, TEMPLATE_FETCH_TIMEOUT_SECS,
    TemplateFetchArgs, download_template_bytes, extract_tarball_into, layer_rel,
    lock_template_cache_layer, read_template_cache_version, register_fetch_auth,
    registry_package_name, select_registry, template_cache_is_link, template_layer_has_contract,
    template_registry_url, templates_cache_dir, validate_template_identity,
    verify_template_integrity,
};
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

/// Fetch template layer về cache registry.
pub async fn fetch(args: TemplateFetchArgs) -> Result<PathBuf> {
    let (path, _) = fetch_layer_at(args, &templates_cache_dir(), None).await?;
    Ok(path)
}

pub(super) async fn fetch_layer_at(
    args: TemplateFetchArgs,
    cache_root: &Path,
    auth_override: Option<&mgc_publish::auth::Auth>,
) -> Result<(PathBuf, String)> {
    let name = registry_package_name(&args.core, &args.name);
    let (path, version, _) = resolve_layer_at(args, cache_root, auth_override, false).await?;
    println!("Template '{}' fetched → {}", name, path.display());
    Ok((path, version))
}

pub(in crate::commands::template) async fn resolve_layer_at(
    args: TemplateFetchArgs,
    cache_root: &Path,
    auth_override: Option<&mgc_publish::auth::Auth>,
    reuse_current_version: bool,
) -> Result<(PathBuf, String, bool)> {
    validate_template_identity(&args.core, &args.name)?;
    let registry = select_registry(args.registry.as_deref())?;
    let name = registry_package_name(&args.core, &args.name);
    let tag = args.tag.unwrap_or_else(|| "latest".to_string());

    let registry_url = template_registry_url(&registry)?;
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(TEMPLATE_FETCH_TIMEOUT_SECS))
        .build()
        .map_err(|_| error::template_request_failed())?;
    let auto = match auth_override {
        Some(auth) => auth.clone(),
        None => register_fetch_auth(&registry).await,
    };
    let authorization = auto.header_value();
    let mut metadata_url = registry_url.clone();
    metadata_url
        .path_segments_mut()
        .map_err(|_| error::template_download_url_invalid())?
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
        serde_json::from_slice(&metadata).map_err(|_| error::template_metadata_invalid())?;

    let versions = meta
        .get("versions")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| error::registry_missing_field("versions"))?;
    let version = versions
        .get(&tag)
        .map(|_| tag.as_str())
        .or_else(|| {
            meta.get("dist-tags")
                .and_then(|tags| tags.get(&tag))
                .and_then(serde_json::Value::as_str)
        })
        .ok_or_else(|| error::registry_missing_field(&format!("dist-tags/{tag}")))?;
    semver::Version::parse(version).map_err(|_| error::template_metadata_invalid())?;
    let dist = versions
        .get(version)
        .and_then(|entry| entry.get("dist"))
        .ok_or_else(|| error::registry_missing_field("version.dist"))?;
    let tarball = dist
        .get("tarball")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| error::registry_missing_field("dist.tarball"))?;
    let integrity = dist
        .get("integrity")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| error::registry_missing_field("dist.integrity"))?;
    fs::create_dir_all(cache_root)?;
    if template_cache_is_link(&fs::symlink_metadata(cache_root)?) {
        return Err(error::template_cache_unsafe_path(cache_root));
    }
    let cache_root = cache_root.canonicalize()?;
    let rel = layer_rel(&args.core, &args.name);
    let target = cache_root.join(&rel);
    let _cache_lock = lock_template_cache_layer(&cache_root, &rel)?;
    if reuse_current_version
        && template_layer_has_contract(&target)?
        && read_template_cache_version(&target)?.as_deref() == Some(version)
    {
        return Ok((target, version.to_string(), true));
    }
    let tarball_url = registry_url
        .join(tarball)
        .map_err(|_| error::template_download_url_invalid())?;
    let bytes = download_template_bytes(
        &client,
        tarball_url,
        &registry_url,
        authorization.as_deref(),
        MAX_TEMPLATE_TARBALL_BYTES,
    )
    .await?;
    verify_template_integrity(&bytes, integrity)?;

    extract_tarball_into(&bytes, &target, Some(version))?;
    Ok((target, version.to_string(), false))
}
