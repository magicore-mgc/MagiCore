//! Fetch one Go release identity, dependency list, integrity markers and result.
//! Lấy identity, dependency, marker integrity và kết quả của một release Go.

use crate::protocols::go::{GO_MOD_SHA256_MARKER_PREFIX, GoInfo, GoModProtocol, parse_go_mod};
use crate::protocols::{ResolvedEntry, sha256_hex};
use mgc_types::{MgError, MgResult};

fn parse_release_identity(module: &str, version: &str, body: &str) -> MgResult<GoInfo> {
    let info = serde_json::from_str::<GoInfo>(body).map_err(|error| {
        MgError::Integrity(format!(
            "Go release metadata is malformed for {module}@{version}: {error}"
        ))
    })?;
    let expected_info_version = format!("v{version}");
    if info.version != expected_info_version {
        return Err(MgError::Integrity(format!(
            "Go release metadata identifies '{}' for requested {module}@{version} (fail-closed)",
            info.version
        )));
    }
    Ok(info)
}

impl GoModProtocol {
    /// Resolve one known version into its immutable registry entry.
    /// Resolve một version đã biết thành registry entry bất biến.
    pub(in crate::protocols::go) async fn resolve_version(
        &self,
        module: &str,
        version: &str,
    ) -> MgResult<ResolvedEntry> {
        let version = version.to_string();
        let artifact_url = self.proxy_url_for(module, &format!("v{version}.zip"));
        let mut markers = Vec::new();

        let info_url = self.proxy_url_for(module, &format!("v{version}.info"));
        let (info_status, info_body) = self.get_text(&info_url).await?;
        if !(200..300).contains(&info_status) {
            return Err(MgError::Network(format!(
                "GET {info_url} returned {info_status} — Go release identity is required (fail-closed)"
            )));
        }
        let info = parse_release_identity(module, &version, &info_body)?;
        if !info.time.is_empty() {
            markers.push(format!("time:{}", info.time));
        }

        let mod_url = self.proxy_url_for(module, &format!("v{version}.mod"));
        let (mod_status, mod_body) = self.get_text(&mod_url).await?;
        let mut deps = Vec::new();
        if (200..300).contains(&mod_status) {
            markers.push(format!(
                "{GO_MOD_SHA256_MARKER_PREFIX}{}",
                sha256_hex(mod_body.as_bytes())
            ));
            let gomod = parse_go_mod(&mod_body)?;
            for req in &gomod.requires {
                if req.indirect {
                    markers.push(format!("indirect:{}", req.path));
                }
                // Dependency replace/exclude directives do not rewrite the main module graph.
                // Chỉ replace/exclude của main module mới tác động graph dự án.
                deps.push((
                    req.path.clone(),
                    req.version.trim_start_matches('v').to_string(),
                ));
            }
        } else {
            return Err(MgError::Network(format!(
                "GET {mod_url} returned {mod_status} — go.mod is required to build the graph (fail-closed)"
            )));
        }

        let sha256 = match self.fetch_ziphash(module, &format!("v{version}")).await? {
            Some(hash) => hash,
            None => {
                let zip_hash = self
                    .fetch_sumdb_ziphash(module, &format!("v{version}"))
                    .await?;
                markers.push(format!("sumdb-ziphash:{zip_hash}"));
                String::new()
            }
        };

        Ok(ResolvedEntry {
            name: module.to_string(),
            version,
            deps,
            artifact_url,
            sha256,
            extra_markers: markers,
        })
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
