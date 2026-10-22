//! Go proxy `.info` and `@latest` JSON models and identity validation.
//! Model JSON `.info`/`@latest` của Go proxy và kiểm tra định danh.

use mgc_types::{MgError, MgResult};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(in crate::protocols::go) struct GoInfo {
    #[serde(default)]
    pub(in crate::protocols::go) version: String,
    #[serde(default)]
    pub(in crate::protocols::go) time: String,
}

pub(in crate::protocols::go) fn validate_go_info(
    bytes: &[u8],
    expected_version: &str,
) -> MgResult<()> {
    let info = serde_json::from_slice::<GoInfo>(bytes).map_err(|error| {
        MgError::Integrity(format!("Go release metadata is malformed: {error}"))
    })?;
    if info.version != expected_version {
        return Err(MgError::Integrity(format!(
            "Go release metadata identifies '{}' but the lock pins '{expected_version}' (fail-closed)",
            info.version
        )));
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(in crate::protocols::go) struct GoLatest {
    #[serde(default)]
    pub(in crate::protocols::go) version: String,
}
