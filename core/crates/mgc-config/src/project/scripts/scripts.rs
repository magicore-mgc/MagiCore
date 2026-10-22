//! Lifecycle script policy DTO, decision rules and table loading.
//! DTO policy script, quy tắc quyết định và đọc bảng cấu hình.

use super::error::ScriptsPolicyError;
use crate::project::read_regular_project_text;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Lifecycle script policy — `[scripts] policy/allow/deny`.
/// npm parity (approve-scripts/deny-scripts) in committed form: the local
/// trust DB stays machine-private, this file is reviewed like code.
/// Policy lifecycle script commit được, tách khỏi trust DB riêng của máy.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScriptsPolicy {
    /// Default for packages with scripts but no explicit entry: `allow`, `deny`, or `prompt`.
    /// Mặc định cho package có script nhưng chưa có entry tường minh.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// Explicitly allowed package names (exact or `name@version`).
    /// Tên package được cho phép tường minh (tên chính xác hoặc `name@version`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allow: Vec<String>,
    /// Explicitly denied package names (exact or `name@version`).
    /// Tên package bị chặn tường minh (tên chính xác hoặc `name@version`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deny: Vec<String>,
}

impl ScriptsPolicy {
    /// Resolve committed file policy: deny wins, then allow, else no opinion.
    /// Quyết định file policy: deny thắng, tiếp theo allow, sau cùng là chưa có ý kiến.
    pub fn decide(&self, name: &str, version: &str) -> Option<(bool, &'static str)> {
        if policy_lists(&self.deny, name, version) {
            return Some((false, "mgc.toml [scripts] deny"));
        }
        if policy_lists(&self.allow, name, version) {
            return Some((true, "mgc.toml [scripts] allow"));
        }
        match self.policy.as_deref() {
            Some("deny") | Some("prompt") => Some((false, "mgc.toml [scripts] policy")),
            // An explicit allow-all is deliberate; absence means no opinion.
            // Allow-all tường minh là chủ ý; thiếu key nghĩa là chưa có ý kiến.
            Some("allow") => Some((true, "mgc.toml [scripts] policy")),
            _ => None,
        }
    }
}

/// True when a package or exact `name@version` appears in a policy list.
/// Cho biết package hoặc `name@version` có trong danh sách policy.
pub fn policy_lists(entries: &[String], name: &str, version: &str) -> bool {
    entries.iter().any(|entry| {
        let entry = entry.trim();
        entry == name || entry == format!("{name}@{version}").as_str()
    })
}

/// Merged lifecycle decision; either deny source takes precedence over allow.
/// Gộp quyết định lifecycle; deny từ nguồn nào cũng thắng allow.
pub enum ScriptVerdict {
    /// Scripts must not run — không chạy script.
    Deny(&'static str),
    /// Scripts may run — được phép chạy script.
    Allow(&'static str),
    /// Neither source has an opinion — chưa nguồn nào quyết định.
    Undecided,
}

/// Apply the shared file-policy and machine trust decision once for all callers.
/// Áp dụng một lần policy file và trust máy dùng chung cho mọi caller.
pub fn decide_scripts(
    name: &str,
    version: &str,
    file_policy: Option<&ScriptsPolicy>,
    db_policy: Option<&str>,
    blanket_scripts: bool,
) -> ScriptVerdict {
    let file_deny = file_policy
        .map(|policy| policy_lists(&policy.deny, name, version))
        .unwrap_or(false);
    if file_deny {
        return ScriptVerdict::Deny("mgc.toml [scripts] deny");
    }
    if db_policy == Some("denied") {
        return ScriptVerdict::Deny("mgc trust deny");
    }
    if let Some((allowed, reason)) = file_policy.and_then(|policy| policy.decide(name, version)) {
        if allowed {
            return ScriptVerdict::Allow(reason);
        }
        return ScriptVerdict::Deny(reason);
    }
    match db_policy {
        Some("approved") => ScriptVerdict::Allow("mgc trust approve"),
        _ if blanket_scripts => ScriptVerdict::Allow("default policy"),
        _ => ScriptVerdict::Undecided,
    }
}

/// Load only `[scripts]`; missing file/table stays `None`, malformed policy fails closed.
/// Chỉ đọc `[scripts]`; thiếu file/table trả `None`, policy lỗi thì fail-closed.
pub fn load_scripts_table(
    project_root: &Path,
) -> Result<Option<ScriptsPolicy>, ScriptsPolicyError> {
    let path = project_root.join("mgc.toml");
    let Some(text) = read_regular_project_text(&path, "project config").map_err(|source| {
        ScriptsPolicyError::ReadProjectConfig {
            path: path.clone(),
            source,
        }
    })?
    else {
        return Ok(None);
    };
    let value: toml::Value =
        toml::from_str(&text).map_err(|source| ScriptsPolicyError::InvalidToml {
            path: path.clone(),
            source,
        })?;
    let Some(table) = value.get("scripts") else {
        return Ok(None);
    };
    let json =
        serde_json::to_value(table).map_err(|source| ScriptsPolicyError::EncodeScriptsTable {
            path: path.clone(),
            source,
        })?;
    serde_json::from_value::<ScriptsPolicy>(json)
        .map(Some)
        .map_err(|source| ScriptsPolicyError::DecodeScriptsTable { path, source })
}
