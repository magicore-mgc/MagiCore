//! NuGet HTTP client calls and flat-container URL construction.
//! HTTP client và tạo URL flat-container NuGet.

use crate::protocols::nuget::NuGetProtocol;
use mgc_types::{MgError, MgResult};

impl NuGetProtocol {
    pub(in crate::protocols::nuget) async fn get_text(&self, url: &str) -> MgResult<(u16, String)> {
        let resp = self
            .client
            .get(url)
            .await
            .map_err(|e| MgError::Network(format!("GET {url} failed: {e}")))?;
        let status = resp.status().as_u16();
        let body = resp
            .text()
            .await
            .map_err(|e| MgError::Network(format!("read {url} body failed: {e}")))?;
        Ok((status, body))
    }

    pub(in crate::protocols::nuget) async fn get_bytes(&self, url: &str) -> MgResult<Vec<u8>> {
        let resp = self
            .client
            .get(url)
            .await
            .map_err(|e| MgError::Network(format!("GET {url} failed: {e}")))?;
        let status = resp.status();
        let bytes = resp
            .bytes()
            .await
            .map_err(|e| MgError::Network(format!("read {url} body failed: {e}")))?;
        if !status.is_success() {
            return Err(MgError::Network(format!("GET {url} returned {status}")));
        }
        Ok(bytes.to_vec())
    }

    pub(in crate::protocols::nuget) fn nupkg_url(&self, id: &str, version: &str) -> String {
        // The NuGet flat-container filename is lowercased like its URL segments.
        // Tên file flat-container NuGet phải viết thường như các đoạn URL.
        let id_lower = id.to_lowercase();
        let version_lower = version.to_lowercase();
        format!(
            "{}/{}/{}/{}.{}.nupkg",
            self.package_base, id_lower, version_lower, id_lower, version_lower
        )
    }

    pub(in crate::protocols::nuget) fn nuspec_url(&self, id: &str, version: &str) -> String {
        format!(
            "{}/{}/{}/{}.nuspec",
            self.package_base,
            id.to_lowercase(),
            version.to_lowercase(),
            id.to_lowercase()
        )
    }
}
