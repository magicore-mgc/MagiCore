//! Exact-pin, version-list and @latest selection for Go modules.
//! Chọn exact-pin, danh sách version và @latest cho Go module.

use crate::protocols::ResolvedEntry;
use crate::protocols::go::{GoLatest, GoModProtocol, go_matches, parse_go_version};
use mgc_types::{MgError, MgResult, Version};

impl GoModProtocol {
    /// Bare exact pin from a range string; returns the raw version without `v`.
    /// Nhận diện version pin chính xác và trả chuỗi không có tiền tố `v`.
    pub(in crate::protocols::go) fn exact_pin_version(range: &str) -> Option<String> {
        let mut raw = range.trim();
        raw = raw.strip_prefix('=').unwrap_or(raw);
        raw = raw.strip_prefix('v').unwrap_or(raw);
        if raw.is_empty() || raw.contains([',', '|', ' ', '\t', '*', '<', '>', '^', '~', '=', '!'])
        {
            return None;
        }
        // Parse to validate the version while preserving its original text.
        // Parse để kiểm tra version nhưng vẫn giữ nguyên chuỗi ban đầu.
        parse_go_version(raw).ok()?;
        Some(raw.to_string())
    }

    /// Probe an exact pin through its `.info`; transport failures propagate.
    /// Kiểm tra exact pin qua `.info`; lỗi truyền tải được giữ nguyên.
    pub(in crate::protocols::go) async fn direct_pin_exists(
        &self,
        module: &str,
        version: &str,
    ) -> MgResult<bool> {
        let url = self.proxy_url_for(module, &format!("v{version}.info"));
        let (status, _) = self.get_text(&url).await?;
        Ok((200..300).contains(&status))
    }

    /// Resolve one requirement while retaining the original fast-path/fallback order.
    /// Resolve requirement và giữ nguyên thứ tự fast-path/fallback hiện tại.
    pub(in crate::protocols::go::resolve) async fn resolve_by_requirement(
        &self,
        name: &str,
        range: &str,
    ) -> MgResult<ResolvedEntry> {
        let module = name.trim();
        if let Some(pinned) = Self::exact_pin_version(range)
            && self.direct_pin_exists(module, &pinned).await?
        {
            return self.resolve_version(module, &pinned).await;
        }
        let list_url = self.proxy_url_for(module, "list");
        let (status, body) = self.get_text(&list_url).await?;
        let mut best: Option<(Version, String)> = None;
        if (200..300).contains(&status) {
            for line in body.lines() {
                let v = line.trim().trim_start_matches('v');
                if line.trim().starts_with("v0.0.0-") || v.is_empty() {
                    continue;
                }
                let Ok(version) = parse_go_version(v) else {
                    continue;
                };
                if !go_matches(range, &version) {
                    continue;
                }
                if best.as_ref().is_none_or(|(bv, _)| version > *bv) {
                    best = Some((version, v.to_string()));
                }
            }
        }
        if best.is_none() {
            let latest_url = self.proxy_url_for(module, "latest");
            let (latest_status, latest_body) = self.get_text(&latest_url).await?;
            if (200..300).contains(&latest_status) {
                let latest: GoLatest = serde_json::from_str(&latest_body)
                    .map_err(|e| MgError::Other(format!("parse go @latest failed: {e}")))?;
                let v = latest.version.trim_start_matches('v').to_string();
                if let Ok(version) = parse_go_version(&v)
                    && go_matches(range, &version)
                {
                    best = Some((version, v));
                }
            }
        }
        let (_, version) = best.ok_or_else(|| {
            MgError::Other(format!(
                "no version of module {module} matches range '{range}'"
            ))
        })?;
        self.resolve_version(module, &version).await
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
