//! Security policy settings and configuration readers.
//! Thiết lập policy bảo mật và đọc cấu hình.
use super::super::file_access::read_regular_project_text;
use super::super::{ScriptsPolicy, load_scripts_table_typed};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Security config — `[security] min_release_age` per ecosystem (quarantine guard).
/// Bảo mật — `[security] min_release_age` theo ecosystem (guard cách ly).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecurityConfig {
    /// Minimum release age in HOURS (global default) — Tuổi tối thiểu gói phát hành, tính bằng GIỜ (mặc định toàn cục)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_release_age: Option<u64>,
    /// When true, versions with missing/unparsable registry timestamps
    /// are KEPT (fail-open) — escape hatch for private registries that
    /// omit `time`. Default false: unstamped versions are rejected with
    /// a clear error naming the package (fail-closed).
    /// (Cho phép version thiếu timestamp — escape hatch cho registry nội bộ.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_missing_time: Option<bool>,

    /// Per-ecosystem min_release_age overrides — Ghi đè min_release_age theo ecosystem
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ai: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lib: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iot: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cicd: Option<u64>,

    /// Per-core minimum release age for hardware artifact lanes.
    /// Tuổi phát hành tối thiểu riêng cho lane artifact hardware.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hardware: Option<u64>,

    /// Explicit per-ecosystem unsigned-artifact escape (`"*" rejected —
    /// see `unsigned_artifact_allowed`). Wired into lanes in Phase C;
    /// today the resolver denies unsigned artifacts unconditionally.
    /// (Escape artifact-không-chữ-ký per-ecosystem tường minh.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_unsigned_artifacts: Option<Vec<String>>,
}

/// Check an unsigned-artifact escape list (`mgc.toml [security]
/// allow_unsigned_artifacts`, mirrored by `MGC_ALLOW_UNSIGNED_ARTIFACTS`
/// comma env): explicit per-ecosystem opt-outs only — `"*"` is rejected
/// as a value (a global opt-out must be a deliberate empty-vs-absent
/// decision at the call site, never a wildcard in config).
/// (Kiểm tra danh sách escape artifact-không-chữ-ký: chỉ opt-out
/// per-ecosystem tường minh — từ chối `"*"`.)
pub fn unsigned_artifact_allowed(allowed: &[String], ecosystem: &str) -> bool {
    allowed
        .iter()
        .any(|entry| entry.trim().eq_ignore_ascii_case(ecosystem))
}

impl SecurityConfig {
    /// Get min_release_age for specific ecosystem (fallback to global) — Lấy min_release_age cho ecosystem cụ thể
    pub fn min_age_for_ecosystem(&self, ecosystem: &str) -> Option<u64> {
        match ecosystem {
            "web" => self.web.or(self.min_release_age),
            "ai" => self.ai.or(self.min_release_age),
            "app" => self.app.or(self.min_release_age),
            "lib" => self.lib.or(self.min_release_age),
            "game" => self.game.or(self.min_release_age),
            "iot" => self.iot.or(self.min_release_age),
            "cloud" | "clo" => self.cloud.or(self.min_release_age),
            "cicd" => self.cicd.or(self.min_release_age),
            "hardware" => self.hardware.or(self.min_release_age),
            _ => self.min_release_age,
        }
    }
}

impl SecurityConfig {
    /// Whether this project explicitly opted OUT of the release-age gate.
    ///
    /// Only an explicit `0` disables quarantine: a missing value means "no
    /// opinion", and the caller (resolver guard / scaffold selector) applies
    /// `DEFAULT_MIN_RELEASE_AGE_HOURS` for it. Treating "absent" as "off" is
    /// exactly the bug that let a typosquat install on a default project.
    /// (Chỉ `0` tường minh mới tắt quarantine. Giá trị thiếu nghĩa là "không ý
    /// kiến" và caller áp `DEFAULT_MIN_RELEASE_AGE_HOURS`. Coi "thiếu" là "tắt"
    /// chính là lỗi khiến typosquat cài được trên project mặc định.)
    pub fn release_age_disabled(&self, ecosystem: &str) -> bool {
        self.min_age_for_ecosystem(ecosystem) == Some(0)
    }

    /// The quarantine in hours that actually applies, or `None` when disabled.
    ///
    /// Precedence: an explicit per-environment value (including `0`) wins;
    /// otherwise the documented default applies.
    /// (Quarantine theo giờ thực sự áp dụng, hoặc `None` khi bị tắt. Giá trị
    /// tường minh thắng, còn lại áp mặc định.)
    pub fn effective_release_age_hours(&self, ecosystem: &str) -> Option<u64> {
        self.min_age_for_ecosystem(ecosystem)
            .or(Some(crate::project::DEFAULT_MIN_RELEASE_AGE_HOURS))
            .filter(|hours| *hours > 0)
    }
}

/// Load only the `[security]` table from mgc.toml, including minimal configs.
/// Chỉ nạp bảng `[security]` từ mgc.toml, kể cả config tối giản.
pub fn load_security_config(project_root: &Path) -> Result<Option<SecurityConfig>, anyhow::Error> {
    let path = project_root.join("mgc.toml");
    let Some(text) = read_regular_project_text(&path, "project config")? else {
        return Ok(None);
    };
    let document: toml::Value = toml::from_str(&text)?;
    let Some(value) = document.get("security") else {
        return Ok(None);
    };
    Ok(Some(value.clone().try_into()?))
}

/// Load the committed `[scripts]` table through the legacy String-error API.
/// Đọc bảng `[scripts]` qua API tương thích hiện dùng lỗi dạng String.
pub fn load_scripts_table(project_root: &Path) -> Result<Option<ScriptsPolicy>, String> {
    load_scripts_table_typed(project_root).map_err(|error| error.to_string())
}
