use mgc_types::{MgError, MgResult};

use super::super::normalize_python_name;
use crate::scanners::osv;

const PYPI_SIMPLE_INDEX: &str = "https://pypi.org/simple";

/// Read uv.lock package pins; unknown registries remain explicitly skipped.
/// Đọc pin từ uv.lock; registry không biết phải được ghi nhận là skipped.
pub fn read_uv_lock_pins(raw: &str) -> MgResult<(Vec<osv::OsvPin>, Vec<String>)> {
    let document: toml::Value = toml::from_str(raw)
        .map_err(|error| MgError::Other(format!("invalid uv.lock TOML: {error}")))?;
    if document.get("version").and_then(toml::Value::as_integer) != Some(1)
        || document
            .get("revision")
            .and_then(toml::Value::as_integer)
            .is_none()
    {
        return Err(MgError::Other(
            "uv.lock has an unsupported or incomplete schema version".to_string(),
        ));
    }
    let packages = match document.get("package") {
        Some(value) => value
            .as_array()
            .ok_or_else(|| MgError::Other("uv.lock package field is not an array".to_string()))?,
        None => return Ok((Vec::new(), Vec::new())),
    };
    let mut pins = std::collections::BTreeSet::new();
    let mut skipped = Vec::new();
    if document
        .get("resolution-markers")
        .and_then(toml::Value::as_array)
        .is_some_and(|markers| !markers.is_empty())
    {
        skipped.push("uv.lock contains platform resolution markers; all locked variants are queried conservatively".to_string());
    }
    for package in packages {
        let name = package
            .get("name")
            .and_then(toml::Value::as_str)
            .ok_or_else(|| MgError::Other("uv.lock package has no valid name".to_string()))?;
        let version = package
            .get("version")
            .and_then(toml::Value::as_str)
            .ok_or_else(|| {
                MgError::Other(format!("uv.lock package '{name}' has no valid version"))
            })?;
        let registry = package
            .get("source")
            .and_then(|source| source.get("registry"))
            .and_then(toml::Value::as_str);
        match registry {
            Some(url) if url.trim_end_matches('/') == PYPI_SIMPLE_INDEX => {
                pins.insert((normalize_python_name(name), version.to_string()));
            }
            Some(url) => skipped.push(format!(
                "uv.lock package '{name}@{version}' uses unsupported registry '{url}'"
            )),
            None => skipped.push(format!(
                "uv.lock package '{name}@{version}' has a non-PyPI or missing source"
            )),
        }
    }
    Ok((
        pins.into_iter()
            .map(|(name, version)| osv::OsvPin {
                name,
                version,
                ecosystem: "PyPI",
            })
            .collect(),
        skipped,
    ))
}
