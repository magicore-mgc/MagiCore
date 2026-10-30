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
    // One alias table (`mgc_types::Ecosystem`); the local copy here was the
    // CLI-side duplicate of the marker writer's `if core == "cloud"` (H-1).
    // (Một bảng alias duy nhất; bản copy ở đây trùng với bản trong marker.)
    let core = core.trim().to_ascii_lowercase();
    mgc_types::Ecosystem::canonical_core_name(&core)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
