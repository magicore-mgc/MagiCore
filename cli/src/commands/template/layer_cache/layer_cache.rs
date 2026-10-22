use super::error;
use crate::commands::template::{
    TemplateFetchArgs, ensure_template_cache_parent, read_template_cache_version, resolve_layer_at,
    template_cache_is_link, template_layer_has_contract, templates_cache_dir,
    validate_template_identity,
};
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

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

/// Validate any existing local layer path without fetching a missing layer.
/// Kiểm tra layer cache đã tồn tại mà không tải layer còn thiếu.
#[cfg(test)]
pub(crate) fn validate_cached_layer_path_in(
    rel: &str,
    configured_cache_root: &Path,
) -> Result<(), crate::scaffold::resolver::ScaffoldResolveError> {
    use crate::scaffold::resolver::ScaffoldResolveError;

    let (core, _) = rel.split_once('/').ok_or_else(|| {
        ScaffoldResolveError::Other(error::template_layer_identity_invalid().to_string())
    })?;
    validate_template_identity(core, rel)
        .map_err(|error| ScaffoldResolveError::Other(error.to_string()))?;

    let cache_root = match fs::symlink_metadata(configured_cache_root) {
        Ok(metadata) if template_cache_is_link(&metadata) || !metadata.is_dir() => {
            return Err(ScaffoldResolveError::Other(
                error::template_cache_unsafe_path(configured_cache_root).to_string(),
            ));
        }
        Ok(_) => configured_cache_root
            .canonicalize()
            .map_err(|error| ScaffoldResolveError::Other(error.to_string()))?,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(ScaffoldResolveError::Other(error.to_string())),
    };

    let mut path = cache_root.clone();
    for component in Path::new(rel).components() {
        let std::path::Component::Normal(component) = component else {
            return Err(ScaffoldResolveError::Other(
                error::template_layer_identity_invalid().to_string(),
            ));
        };
        path.push(component);
        match fs::symlink_metadata(&path) {
            Ok(metadata) if template_cache_is_link(&metadata) || !metadata.is_dir() => {
                return Err(ScaffoldResolveError::Other(
                    error::template_cache_unsafe_path(&path).to_string(),
                ));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(error) => return Err(ScaffoldResolveError::Other(error.to_string())),
        }
    }

    template_layer_has_contract(&path)
        .map_err(|error| ScaffoldResolveError::Other(error.to_string()))?;
    read_template_cache_version(&path)
        .map_err(|error| ScaffoldResolveError::Other(error.to_string()))?;
    Ok(())
}

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

pub(super) fn template_cache_matches_ref(
    requested: &crate::scaffold::spec::ScaffoldRef,
    cached: Option<&str>,
) -> bool {
    use crate::scaffold::spec::ScaffoldRef;
    match requested {
        ScaffoldRef::Default => false,
        ScaffoldRef::Version(version) => cached == Some(version.as_str()),
        ScaffoldRef::DistTag(_) | ScaffoldRef::Range(_) => false,
    }
}

pub(in crate::commands::template) fn template_resolution_error(
    error: anyhow::Error,
    layer: &str,
    core: &str,
    name: &str,
    tag: &str,
) -> crate::scaffold::resolver::ScaffoldResolveError {
    use crate::scaffold::resolver::ScaffoldResolveError;
    if error.is::<error::TemplateTransportUnavailable>() {
        return ScaffoldResolveError::RegistryUnavailable {
            registry: "configured template registry".to_string(),
            offline: false,
        };
    }
    if let Some(status) = error.downcast_ref::<error::TemplateHttpStatus>() {
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
