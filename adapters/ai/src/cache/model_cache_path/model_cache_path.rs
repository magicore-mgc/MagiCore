//! Resolve a validated model identifier below its registry cache — dựng path nằm trong cache registry.

use super::error::ModelCachePathError;
use crate::cache::cache_dir;
use crate::registry::Registry;
use mgc_types::MgResult;
use std::path::PathBuf;

/// Map a registry model ID to the existing cache layout after rejecting path syntax.
/// Tạo path theo layout cache hiện tại sau khi từ chối cú pháp có thể thoát khỏi cache.
pub fn model_cache_path(registry: &Registry, model_id: &str) -> MgResult<PathBuf> {
    validate_model_id(model_id)?;
    let base = cache_dir(registry)?;
    Ok(base.join(cache_entry_name(registry, model_id)))
}

pub(in crate::cache) fn cache_entry_name(registry: &Registry, model_id: &str) -> String {
    let flattened = model_id.replace('/', "--");
    if matches!(registry, Registry::HuggingFace) {
        format!("models--{flattened}")
    } else {
        flattened
    }
}

pub(in crate::cache) fn model_id_for_cache_entry(
    registry: &Registry,
    entry_name: &str,
    is_directory: bool,
) -> Option<String> {
    if entry_name.to_ascii_lowercase().ends_with(".meta.json") {
        return None;
    }

    match registry {
        Registry::HuggingFace => {
            if !is_directory {
                return None;
            }
            let flattened = entry_name.strip_prefix("models--")?;
            let model_id = flattened.replace("--", "/");
            validate_model_id(&model_id).ok()?;
            Some(model_id)
        }
        _ => Some(entry_name.replace("--", "/")),
    }
}

fn validate_model_id(model_id: &str) -> Result<(), ModelCachePathError> {
    if model_id.is_empty() || model_id.to_ascii_lowercase().ends_with(".meta.json") {
        return Err(ModelCachePathError::InvalidModelId);
    }

    if model_id.chars().any(|character| {
        character.is_control()
            || matches!(character, '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|')
    }) {
        return Err(ModelCachePathError::InvalidModelId);
    }

    for segment in model_id.split('/') {
        if segment.is_empty()
            || segment == "."
            || segment == ".."
            || segment.ends_with('.')
            || segment.ends_with(' ')
            || is_windows_device_name(segment)
        {
            return Err(ModelCachePathError::InvalidModelId);
        }
    }

    Ok(())
}

fn is_windows_device_name(segment: &str) -> bool {
    let stem = segment
        .split_once('.')
        .map_or(segment, |(stem, _)| stem)
        .trim_end_matches(['.', ' '])
        .to_ascii_uppercase();

    matches!(
        stem.as_str(),
        "CON" | "PRN" | "AUX" | "NUL" | "CONIN$" | "CONOUT$"
    ) || ["COM", "LPT"].iter().any(|prefix| {
        stem.strip_prefix(prefix).is_some_and(|number| {
            matches!(
                number,
                "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³"
            )
        })
    })
}
