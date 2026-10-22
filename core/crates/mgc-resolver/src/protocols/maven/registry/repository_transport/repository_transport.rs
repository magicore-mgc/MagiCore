//! Build Maven repository URLs and perform typed HTTP reads.
//! Dựng URL repository Maven và thực hiện HTTP read có lỗi typed dùng chung.
use crate::protocols::maven::registry::registry::MavenProtocol;
use mgc_types::{MgError, MgResult};

impl MavenProtocol {
    /// `{repo}/{group-path}/{artifact}/{version}/{file}` — group
    /// `com.example.foo` → path `com/example/foo`.
    /// `{repo}/{group-path}/{artifact}/{version}/{file}` — group
    /// `com.example.foo` → path `com/example/foo`.
    pub(in crate::protocols::maven) fn repo_url_for(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        file: &str,
    ) -> String {
        let gpath = group.replace('.', "/");
        if version.is_empty() {
            format!("{}/{gpath}/{artifact}/{file}", self.repo_url)
        } else {
            format!("{}/{gpath}/{artifact}/{version}/{file}", self.repo_url)
        }
    }

    pub(in crate::protocols::maven) async fn get_text(&self, url: &str) -> MgResult<(u16, String)> {
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

    pub(in crate::protocols::maven) async fn get_bytes(&self, url: &str) -> MgResult<Vec<u8>> {
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

    /// Fetch the POM for a concrete `(group, artifact, version)`.
    /// Tải POM cho `(group, artifact, version)` cụ thể.
    pub async fn download_pom(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
    ) -> MgResult<Vec<u8>> {
        self.get_bytes(&self.repo_url_for(
            group,
            artifact,
            version,
            &format!("{artifact}-{version}.pom"),
        ))
        .await
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
