use crate::LibLanguage;
use crate::adapter::types::LibAdapter;
use mgc_resolver::protocols::PypiProtocol;
use mgc_types::MgResult;
use std::path::Path;

impl LibAdapter {
    pub(in crate::adapter) fn security_age_gate_policy_for(
        &self,
        project_root: &Path,
    ) -> MgResult<Option<(u64, bool)>> {
        let path = project_root.join("mgc.toml");
        let Some(text) = mgc_config::project::read_regular_project_text(&path, "project config")
            .map_err(|error| {
                mgc_types::MgError::Other(format!("cannot read {}: {error}", path.display()))
            })?
        else {
            return Ok(None);
        };
        let document: toml::Value = toml::from_str(&text).map_err(|error| {
            mgc_types::MgError::Other(format!("invalid TOML in {}: {error}", path.display()))
        })?;
        let Some(security_value) = document.get("security") else {
            return Ok(None);
        };
        let security: mgc_config::SecurityConfig =
            security_value.clone().try_into().map_err(|error| {
                mgc_types::MgError::Other(format!(
                    "invalid [security] table in {}: {error}",
                    path.display()
                ))
            })?;
        Ok(security
            .min_age_for_ecosystem(self.security_core)
            .map(|cutoff_hours| (cutoff_hours, security.allow_missing_time.unwrap_or(false))))
    }

    pub(in crate::adapter) fn pypi_protocol_for(
        &self,
        project_root: &Path,
    ) -> MgResult<PypiProtocol> {
        let protocol = PypiProtocol::from_env();
        Ok(match self.security_age_gate_policy_for(project_root)? {
            Some((cutoff_hours, allow_missing_time)) => {
                protocol.with_age_gate(cutoff_hours, allow_missing_time)
            }
            None => protocol,
        })
    }

    pub(in crate::adapter) fn ensure_age_gate_supported(
        &self,
        project_root: &Path,
    ) -> MgResult<()> {
        let Some((cutoff_hours, _)) = self.security_age_gate_policy_for(project_root)? else {
            return Ok(());
        };
        if cutoff_hours == 0 || self.language == LibLanguage::Python || self.web.is_some() {
            return Ok(());
        }
        Err(mgc_types::MgError::Unsupported {
            core: "lib",
            capability: "minimum release age",
            guidance: format!(
                "[security] sets a {cutoff_hours}h minimum release age, but the {} registry lane has no trusted publication timestamps; remove the cutoff or use a timestamp-aware lane",
                self.language()
            ),
        })
    }
}
