//! Fetch package metadata and archives through the shared MGC HTTP client.
//! Tải metadata và archive qua HTTP client dùng chung của MGC.

use super::super::{
    PubProtocol,
    package_catalog::{PubPackage, validate_pub_package_name},
};
use crate::protocols::ResolvedEntry;
/// Largest pub.dev archive accepted into memory.
/// Archive pub.dev lớn nhất được nhận vào bộ nhớ.
///
/// pub.dev publishes packages far below this; the cap exists so a compromised
/// or misconfigured response cannot exhaust memory.
/// (pub.dev publish package nhỏ hơn nhiều so với ngưỡng này; giới hạn tồn tại
/// để response bị chiếm/ cấu hình sai không làm cạn RAM.)
const MAX_PUB_ARCHIVE_BYTES: usize = 128 * 1024 * 1024;

use mgc_types::{MgError, MgResult};

impl PubProtocol {
    async fn get_text(&self, url: &str) -> MgResult<String> {
        let resp = self
            .client
            .get(url)
            .await
            .map_err(|e| MgError::Network(format!("GET {url} failed: {e}")))?;
        let status = resp.status();
        let body = resp
            .text()
            .await
            .map_err(|e| MgError::Network(format!("read {url} body failed: {e}")))?;
        if !status.is_success() {
            return Err(MgError::Network(format!(
                "GET {url} returned {status}: {body}"
            )));
        }
        Ok(body)
    }

    pub(in crate::protocols::pubdev) async fn package_doc(
        &self,
        name: &str,
    ) -> MgResult<PubPackage> {
        validate_pub_package_name(name)?;
        let url = format!("{}/api/packages/{name}", self.api_url);
        let body = self.get_text(&url).await?;
        serde_json::from_str(&body)
            .map_err(|error| MgError::Other(format!("parse pub.dev json failed: {error}")))
    }

    pub(in crate::protocols::pubdev) async fn download_entry(
        &self,
        entry: &ResolvedEntry,
    ) -> MgResult<Vec<u8>> {
        // A pub.dev response had no byte cap: the archive was read into memory
        // whole before any limit applied, so one oversized (or hostile) tarball
        // could take the process down. The bounded helper refuses past the cap
        // while streaming.
        // (Response pub.dev không có giới hạn byte: archive được đọc hết vào RAM
        // trước khi bất kỳ ngưỡng nào áp dụng, nên một tarball quá lớn (hoặc
        // độc hại) có thể hạ cả process. Helper có giới hạn từ chối ngay khi
        // vượt ngưỡng trong lúc stream.)
        let (status, bytes) = self
            .client
            .get_bytes_limited(&entry.artifact_url, MAX_PUB_ARCHIVE_BYTES)
            .await
            .map_err(|e| MgError::Network(format!("GET {} failed: {e}", entry.artifact_url)))?;
        if !(200..300).contains(&status) {
            return Err(MgError::Network(format!(
                "GET {} returned {status}",
                entry.artifact_url
            )));
        }
        Ok(bytes)
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
