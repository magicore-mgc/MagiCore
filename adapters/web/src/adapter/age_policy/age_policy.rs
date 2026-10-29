//! `age_policy.rs` — Project-scoped package age policy / policy tuổi package theo project.
use super::super::WebAdapter;
use mgc_types::{PackageId, PackageName, Version, VersionRange, adapter::AddOptions};
use std::path::Path;

impl WebAdapter {
    /// Load mgc.toml `[security]` age policy for ONE project (P0/F6).
    /// Missing file/table/fields = Ok(None) (no filtering, historical
    /// behavior) — but a PRESENT, broken file or wrongly-typed field is
    /// a hard Err (fail-closed: a swallowed `.ok()` here once let a
    /// believed-active deny vanish). Tolerates minimal mgc.toml files
    /// (no name/ecosystem) by parsing the `[security]` table only.
    /// (Nạp policy tuổi cho một project — file hỏng thì lỗi cứng.)
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
        let path = project_root.join("mgc.toml");
        let Some(text) = mgc_config::project::read_regular_project_text(&path, "project config")
            .map_err(|error| MgError::Other(format!("cannot read {}: {error}", path.display())))?
        else {
            return Ok(None);
        };
        let value: toml::Value = toml::from_str(&text)
            .map_err(|e| MgError::Other(format!("invalid TOML in {}: {e}", path.display())))?;
        let Some(table) = value.get("security") else {
            return Ok(None);
        };
        let security: mgc_config::project::SecurityConfig =
            table.clone().try_into().map_err(|e| {
                MgError::Other(format!(
                    "invalid [security] table in {}: {e}",
                    path.display()
                ))
            })?;
        Ok(security
            .min_age_for_ecosystem(core)
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
