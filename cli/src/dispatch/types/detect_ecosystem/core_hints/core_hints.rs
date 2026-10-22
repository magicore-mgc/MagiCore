//! Normalize and validate explicit core declarations from native manifests.
//! Chuẩn hóa và kiểm tra khai báo core tường minh từ manifest native.

#[path = "error.rs"]
mod error;

use anyhow::Result;
use mgc_config::project::ProjectConfig;
use std::collections::BTreeSet;

pub(in crate::dispatch) fn resolve(declarations: Vec<String>) -> Result<Option<String>> {
    let declared_cores = declarations
        .iter()
        .map(|core| canonical_core_hint(core))
        .collect::<BTreeSet<_>>();

    if declared_cores.len() > 1 {
        return Err(error::CoreHintError::ConflictingManifestCores {
            cores: declared_cores.into_iter().collect::<Vec<_>>().join(", "),
        }
        .into());
    }

    let Some(core) = declared_cores.into_iter().next() else {
        return Ok(None);
    };
    if !ProjectConfig::KNOWN_CORES.contains(&core.as_str()) {
        return Err(error::CoreHintError::UnknownManifestCore { core }.into());
    }
    Ok(Some(core))
}

fn canonical_core_hint(core: &str) -> String {
    let core = core.trim().to_ascii_lowercase();
    if core == "cloud" {
        "clo".to_string()
    } else {
        core
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
