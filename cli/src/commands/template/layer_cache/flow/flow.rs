//! Ensure-layer fetch/resolve flow.
//! Luồng fetch/resolve đảm bảo layer.

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

use crate::commands::template::{
    TemplateFetchArgs, ensure_template_cache_parent, read_template_cache_version, resolve_layer_at,
    template_cache_is_link, template_layer_has_contract, validate_template_identity,
};

use super::super::error;
use super::super::error_mapping::template_resolution_error;
use super::super::ref_match::template_cache_matches_ref;

pub(in crate::commands::template) async fn ensure_layer_in(
    requested_layer: &str,
    configured_cache_root: &Path,
    registry_override: Option<&str>,
    auth_override: Option<&mgc_publish::auth::Auth>,
) -> Result<
    crate::scaffold::resolver::ScaffoldResolveStatus,
    crate::scaffold::resolver::ScaffoldResolveError,
> {
    use crate::scaffold::resolver::{ScaffoldResolveError, ScaffoldResolveStatus};
    use crate::scaffold::spec::{CoreKind, ScaffoldRef, parse_scaffold_spec};

    let (prefix, leaf) = requested_layer.rsplit_once('/').ok_or_else(|| {
        ScaffoldResolveError::Other(error::template_layer_identity_invalid().to_string())
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
    fs::create_dir_all(configured_cache_root)
        .map_err(|error| ScaffoldResolveError::Other(error.to_string()))?;
    if template_cache_is_link(
        &fs::symlink_metadata(configured_cache_root)
            .map_err(|error| ScaffoldResolveError::Other(error.to_string()))?,
    ) {
        return Err(ScaffoldResolveError::Other(
            error::template_cache_unsafe_path(configured_cache_root).to_string(),
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
                        error::template_cache_unsafe_path(path).to_string(),
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
    let cached_version = read_template_cache_version(&visible_path)
        .map_err(|error| ScaffoldResolveError::Other(error.to_string()))?;
    if has_contract
        && (is_dev_override
            || (matches!(&spec.requested_ref, ScaffoldRef::Version(_))
                && cached_version.is_some()
                && template_cache_matches_ref(&spec.requested_ref, cached_version.as_deref())))
    {
        if !is_dev_override {
            ensure_template_cache_parent(visible_path.parent().unwrap_or(&cache_root))
                .map_err(|error| ScaffoldResolveError::Other(error.to_string()))?;
        }
        return Ok(ScaffoldResolveStatus::CacheHit {
            layer: rel.to_string(),
            version: cached_version,
            path: visible_path,
        });
    }
    if has_contract && cached_version.is_none() && !is_dev_override {
        mgc_ui::warning(
            "Ignoring an unversioned local template cache; fetching and verifying the registry artifact",
        );
    }
    if is_dev_override && !has_contract {
        return Err(ScaffoldResolveError::Other(
            error::template_layer_missing_manifest(rel).to_string(),
        ));
    }
    // No compiled processor can stand in for a template layer: framework
    // source always comes from the registry or a local project.
    // Không processor biên dịch nào thay được template layer: source framework
    // luôn đến từ registry hoặc project local.
    // Resolve absent layers from the configured registry only.
    // Layer chưa có chỉ được resolve từ registry đã cấu hình.
    // Use the same npm protocol, configured registry and security boundary as explicit fetch.
    // Tự động tải qua đúng protocol/config/guard của template fetch, vào cache processor đang đọc.
    let tag = match &spec.requested_ref {
        ScaffoldRef::Version(version) | ScaffoldRef::DistTag(version) => version.clone(),
        ScaffoldRef::Default => "latest".to_string(),
        ScaffoldRef::Range(_) => {
            return Err(ScaffoldResolveError::Other(
                error::template_layer_identity_invalid().to_string(),
            ));
        }
    };
    match resolve_layer_at(
        TemplateFetchArgs {
            core: core_str.to_string(),
            name: rel.to_string(),
            registry: registry_override.map(str::to_string),
            tag: Some(tag.clone()),
        },
        &cache_root,
        auth_override,
        true,
    )
    .await
    {
        Ok((path, version, true)) => Ok(ScaffoldResolveStatus::CacheHit {
            layer: rel.to_string(),
            version: Some(version),
            path,
        }),
        Ok((path, version, false)) => Ok(ScaffoldResolveStatus::Fetched {
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
