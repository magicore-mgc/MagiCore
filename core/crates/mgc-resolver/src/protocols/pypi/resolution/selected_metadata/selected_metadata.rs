//! Reuse or fetch selected-release JSON metadata.
//! Tái sử dụng hoặc tải JSON metadata của release đã chọn.

use crate::protocols::pypi::{PypiInfo, PypiJson, PypiProtocol};
use mgc_types::{MgError, MgResult, Version};

pub(in crate::protocols::pypi::resolution) async fn load_selected_metadata(
    protocol: &PypiProtocol,
    name: &str,
    version: &Version,
    selected_info: Option<PypiInfo>,
) -> MgResult<PypiInfo> {
    match selected_info {
        Some(info) => Ok(info),
        None => {
            let version_url = format!("{}/pypi/{name}/{version}/json", protocol.index_url);
            let version_body = protocol.get_text(&version_url).await?;
            let version_doc: PypiJson = serde_json::from_str(&version_body).map_err(|error| {
                MgError::Other(format!("parse pypi version json failed: {error}"))
            })?;
            Ok(version_doc.info)
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
