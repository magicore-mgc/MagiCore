//! `age_policy.rs` — Project-scoped package age policy / policy tuổi package theo project.
use super::super::WebAdapter;
use mgc_types::{PackageId, PackageName, Version, VersionRange, adapter::AddOptions};
use std::path::Path;

impl WebAdapter {
    /// Load the release-age policy that actually applies to ONE project
    /// (P0/F6), so `mgc install` and `mgc versions` agree.
    ///
    /// Precedence: an explicit `MAGICORE_ALLOW_UNTRUSTED` bypass; an explicit
    /// `[security]` value (including `0` = opt out); otherwise the documented
    /// `DEFAULT_MIN_RELEASE_AGE_HOURS` default. The default is what the
    /// resolver guard enforces, and returning it here also arms the provider's
    /// full-packument mode (`time` is absent from abbreviated documents).
    /// A PRESENT, broken file or wrongly-typed field is a hard Err
    /// (fail-closed: a swallowed `.ok()` here once let a believed-active deny
    /// vanish). Tolerates minimal mgc.toml files (no name/ecosystem).
    /// (Nạp policy tuổi thực sự áp dụng cho một project — ưu tiên bypass,
    /// rồi giá trị `[security]` tường minh (kể cả 0), còn lại áp mặc định
    /// `DEFAULT_MIN_RELEASE_AGE_HOURS`. File hỏng thì lỗi cứng.)
    pub fn load_age_policy_for(
        project_root: &std::path::Path,
    ) -> mgc_types::MgResult<Option<crate::provider::AgePolicy>> {
        Self::load_age_policy_for_core(project_root, "web")
    }

    /// Load age policy for the owning core when this engine is embedded.
    /// Nạp policy tuổi theo core sở hữu khi engine này được nhúng.
    pub fn load_age_policy_for_core(
        project_root: &std::path::Path,
        core: &str,
    ) -> mgc_types::MgResult<Option<crate::provider::AgePolicy>> {
        use mgc_types::MgError;
        // The documented escape hatch is checked FIRST so it works for every
        // project shape (absent, malformed, explicit) — an operator who hits a
        // timestamp-less private registry must not be blocked by the file that
        // problem lives in.
        // (Escape hatch tài liệu hóa được kiểm TRƯỚC để dùng được với mọi hình
        // dạng project — người dùng registry nội bộ không timestamp không bị
        // chặn bởi chính file chứa vấn đề.)
        if env_truthy("MAGICORE_ALLOW_UNTRUSTED") {
            return Ok(None);
        }
        let path = project_root.join("mgc.toml");
        let Some(text) = mgc_config::project::read_regular_project_text(&path, "project config")
            .map_err(|error| MgError::Other(format!("cannot read {}: {error}", path.display())))?
        else {
            return Ok(Some(default_age_policy()));
        };
        let value: toml::Value = toml::from_str(&text)
            .map_err(|e| MgError::Other(format!("invalid TOML in {}: {e}", path.display())))?;
        let Some(table) = value.get("security") else {
            return Ok(Some(default_age_policy()));
        };
        let security: mgc_config::project::SecurityConfig =
            table.clone().try_into().map_err(|e| {
                MgError::Other(format!(
                    "invalid [security] table in {}: {e}",
                    path.display()
                ))
            })?;
        Ok(security
            .effective_release_age_hours(core)
            .map(|cutoff_hours| crate::provider::AgePolicy {
                cutoff_hours,
                allow_missing_time: security.allow_missing_time.unwrap_or(false),
            }))
    }

    /// Set the gate using a core-specific override before an operation.
    /// Đặt gate theo override của core trước operation.
    pub fn arm_age_gate_for_core(
        &self,
        project_root: &std::path::Path,
        core: &str,
    ) -> mgc_types::MgResult<()> {
        let policy = Self::load_age_policy_for_core(project_root, core)?;
        self.provider.set_age_policy(policy);
        Ok(())
    }

    /// Resolve-first add under an embedded core's age policy.
    /// Resolve-first add theo policy tuổi của core nhúng.
    pub async fn prepare_add_for_core(
        &self,
        project_root: &Path,
        name: &PackageName,
        range: Option<&VersionRange>,
        opts: AddOptions,
        core: &str,
    ) -> mgc_types::MgResult<mgc_types::adapter::PreparedAdd> {
        self.arm_age_gate_for_core(project_root, core)?;
        let (inferred, selected_version) = self
            .infer_add_range(project_root, name, range, opts.exact)
            .await?;
        let version = selected_version.unwrap_or_else(|| Version::new(0, 0, 0));
        Ok(mgc_types::adapter::PreparedAdd {
            id: PackageId::new(name.clone(), version),
            range: inferred,
        })
    }
}

/// The documented default quarantine, used when a project says nothing.
///
/// A missing publish timestamp stays refused (fail-closed): a registry that
/// cannot say when a release appeared cannot prove it left quarantine.
/// (Quarantine mặc định dùng khi project không nói gì. Thiếu timestamp vẫn bị
/// từ chối: registry không chứng minh được release đã ra khỏi quarantine.)
fn default_age_policy() -> crate::provider::AgePolicy {
    crate::provider::AgePolicy {
        cutoff_hours: mgc_config::DEFAULT_MIN_RELEASE_AGE_HOURS,
        allow_missing_time: false,
    }
}

/// Truthy env flag, shared with the supply-chain bypass semantics.
/// (Cờ env truthy, cùng ngữ nghĩa với bypass supply-chain.)
fn env_truthy(name: &str) -> bool {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
        .is_some_and(|value| matches!(value.as_str(), "1" | "true" | "yes" | "on"))
}
