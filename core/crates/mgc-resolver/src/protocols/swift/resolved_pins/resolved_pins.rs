use crate::protocols::swift::registry::SwiftRegistryProtocol;
use crate::protocols::swift::types::SwiftResolvedPin;
use mgc_types::{MgError, MgResult};
use serde_json::Value;
use std::path::{Path, PathBuf};

/// Import pins from a Package.resolved (v1 object.pins / v2/v3 pins —
/// tolerant: unknown shapes are skipped).
/// Nhập pin từ Package.resolved (v1 object.pins / v2/v3 pins — khoan dung:
/// hình dạng lạ bị bỏ qua).
pub fn parse_package_resolved(text: &str) -> MgResult<Vec<SwiftResolvedPin>> {
    let doc: Value = serde_json::from_str(text)
        .map_err(|e| MgError::Other(format!("parse Package.resolved failed: {e}")))?;
    let pins = doc
        .get("pins")
        .or_else(|| doc.get("object").and_then(|o| o.get("pins")))
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or_default();
    let mut out = Vec::new();
    for pin in pins {
        let identity = pin
            .get("identity")
            .or_else(|| pin.get("package"))
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        if identity.is_empty() {
            continue;
        }
        let state = pin.get("state");
        let get = |k: &str| {
            state
                .and_then(|s| s.get(k))
                .and_then(Value::as_str)
                .map(str::to_string)
        };
        out.push(SwiftResolvedPin {
            identity,
            version: get("version"),
            revision: get("revision"),
            branch: get("branch"),
            location: pin
                .get("location")
                .or_else(|| pin.get("repositoryURL"))
                .and_then(Value::as_str)
                .map(str::to_string),
        });
    }
    Ok(out)
}

impl SwiftRegistryProtocol {
    /// Export a SwiftPM-compatible (v2) `Package.resolved` from resolved
    /// pins. Existing files are overwritten — the resolution is the truth.
    /// Xuất `Package.resolved` tương thích SwiftPM (v2) từ các pin đã
    /// resolve. File có sẵn bị ghi đè — kết quả resolve là chân lý.
    pub fn export_package_resolved(
        &self,
        pins: &[SwiftResolvedPin],
        project_root: &Path,
    ) -> MgResult<PathBuf> {
        let pins_json: Vec<Value> = pins
            .iter()
            .map(|pin| {
                let mut state = serde_json::Map::new();
                if let Some(v) = &pin.version {
                    state.insert("version".into(), Value::String(v.clone()));
                }
                if let Some(r) = &pin.revision {
                    state.insert("revision".into(), Value::String(r.clone()));
                }
                if let Some(b) = &pin.branch {
                    state.insert("branch".into(), Value::String(b.clone()));
                }
                let kind = if pin.location.as_ref().is_some_and(|l| l.contains("://")) {
                    "remote"
                } else {
                    "registry"
                };
                serde_json::json!({
                    "identity": pin.identity,
                    "kind": kind,
                    "location": pin.location.clone().unwrap_or_else(|| {
                        format!("registry+{}/{}", self.registry_base(), pin.identity)
                    }),
                    "state": Value::Object(state),
                })
            })
            .collect();
        let doc = serde_json::json!({ "version": 2, "pins": pins_json });
        let path = project_root.join("Package.resolved");
        std::fs::write(
            &path,
            serde_json::to_string_pretty(&doc).unwrap_or_default() + "\n",
        )
        .map_err(|e| MgError::Other(format!("write Package.resolved: {e}")))?;
        Ok(path)
    }
}
