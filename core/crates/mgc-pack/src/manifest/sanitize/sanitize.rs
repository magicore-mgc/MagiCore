//! Validate and project package JSON into the exportable publish manifest.
//! Kiểm tra và tạo manifest package JSON dùng để publish.

use anyhow::Result;
use serde_json::{Map, Value};

use crate::manifest::{ManifestSanitizeError, Sanitized, name_or, version_or};

/// Sanitize a manifest, rejecting malformed private flags before projection.
/// Làm sạch manifest và chặn private flag sai kiểu trước khi tạo output.
pub fn sanitize(raw: Value, name: &str, version: &str) -> Result<Sanitized> {
    let Some(obj) = raw.as_object() else {
        return Err(ManifestSanitizeError::NotJsonObject.into());
    };

    match obj.get("private") {
        Some(Value::Bool(true)) => return Err(ManifestSanitizeError::PrivatePackage.into()),
        Some(Value::Bool(false)) | None => {}
        Some(_) => return Err(ManifestSanitizeError::InvalidPrivateType.into()),
    }

    let manifest_name = obj
        .get("name")
        .and_then(Value::as_str)
        .map(str::to_string)
        .unwrap_or_default();
    if manifest_name.is_empty() {
        return Err(ManifestSanitizeError::MissingName.into());
    }

    let manifest_version = obj
        .get("version")
        .and_then(Value::as_str)
        .map(str::to_string)
        .unwrap_or_default();
    if manifest_version.is_empty() {
        return Err(ManifestSanitizeError::MissingVersion.into());
    }

    let mut out: Map<String, Value> = obj.clone();
    out.remove("private");
    out.remove("publishConfig");

    if !name.is_empty() {
        out.insert("name".into(), Value::String(name.to_string()));
    }
    if !version.is_empty() {
        out.insert("version".into(), Value::String(version.to_string()));
    }

    Ok(Sanitized {
        manifest: Value::Object(out),
        name: name_or(&manifest_name, name),
        version: version_or(&manifest_version, version),
    })
}
