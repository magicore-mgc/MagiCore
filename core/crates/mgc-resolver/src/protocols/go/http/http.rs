//! Go proxy and checksum-database HTTP exchange.
//! Trao đổi HTTP với Go proxy và checksum database.

use crate::protocols::go::{
    GO_MOD_SHA256_MARKER_PREFIX, GoModProtocol, GoModuleFiles, escape_module_path, validate_go_info,
};
use crate::protocols::{ResolvedEntry, sha256_hex};
use mgc_types::{MgError, MgResult};

impl GoModProtocol {
    pub(in crate::protocols::go) async fn get_text(&self, url: &str) -> MgResult<(u16, String)> {
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

    pub(in crate::protocols::go) async fn get_bytes(&self, url: &str) -> MgResult<Vec<u8>> {
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

    /// `{proxy}/{escaped-module}/@v/{file}` — module paths use the proxy
    /// escape (uppercase → `!lowercase`).
    /// `{proxy}/{module-escaped}/@v/{file}` — path module dùng escape của
    /// proxy (chữ hoa → `!chữ_thường`).
    pub(in crate::protocols::go) fn proxy_url_for(&self, module: &str, file: &str) -> String {
        format!(
            "{}/{}/@v/{}",
            self.proxy_url,
            escape_module_path(module),
            file
        )
    }

    pub(in crate::protocols::go) fn sum_lookup_url(&self, module: &str, version: &str) -> String {
        format!(
            "{}/lookup/{}@{}",
            self.sum_url,
            escape_module_path(module),
            version
        )
    }

    /// Fetch a `.ziphash` (hex sha256 of the zip). `Ok(None)` = the proxy
    /// does not publish one (404) — caller falls back to the sumdb.
    /// Tải `.ziphash` (hex sha256 của zip). `Ok(None)` = proxy không công bố
    /// (404) — caller fallback sang sumdb.
    pub(in crate::protocols::go) async fn fetch_ziphash(
        &self,
        module: &str,
        version: &str,
    ) -> MgResult<Option<String>> {
        let url = self.proxy_url_for(module, &format!("{version}.ziphash"));
        let (status, body) = self.get_text(&url).await?;
        if status == 404 {
            return Ok(None);
        }
        if !(200..300).contains(&status) {
            return Err(MgError::Network(format!("GET {url} returned {status}")));
        }
        let hash = body.trim();
        if hash.len() != 64 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
            // A malformed proxy hash is a fail-closed integrity error, never
            // silently ignored.
            // (Hash proxy sai định dạng là lỗi integrity fail-closed, không
            // bao giờ bỏ qua âm thầm.)
            return Err(MgError::Integrity(format!(
                "malformed .ziphash for {module}@{version}: '{hash}'"
            )));
        }
        Ok(Some(hash.to_ascii_lowercase()))
    }

    /// Sumdb record for the zip: parse the lookup response and return the
    /// `h1:` directory hash of the module zip. The REAL sumdb lookup body
    /// is space-separated tokens — `<module> <version> <zip-h1>` on its
    /// own line (the `/go.mod` line carries `<version>/go.mod` as its
    /// second token and never matches). An earlier revision matched
    /// `{module}@{version}` and rejected EVERY genuine response.
    /// Bản ghi sumdb cho zip: parse response lookup và trả directory hash
    /// `h1:` của zip module. Body sumdb thật là các token cách nhau bằng
    /// dấu cách.
    pub(in crate::protocols::go) async fn fetch_sumdb_ziphash(
        &self,
        module: &str,
        version: &str,
    ) -> MgResult<String> {
        let url = self.sum_lookup_url(module, version);
        let (status, body) = self.get_text(&url).await?;
        if !(200..300).contains(&status) {
            return Err(MgError::Network(format!(
                "sumdb lookup {module}@{version} returned {status} — artifact cannot be verified (fail-closed)"
            )));
        }
        for line in body.lines() {
            let mut tokens = line.split_whitespace();
            match (tokens.next(), tokens.next(), tokens.next()) {
                (Some(record_module), Some(record_version), Some(zip_hash))
                    if record_module == module && record_version == version =>
                {
                    if !zip_hash.starts_with("h1:") {
                        return Err(MgError::Integrity(format!(
                            "sumdb zip hash for {module}@{version} is not an h1 hash: '{zip_hash}'"
                        )));
                    }
                    return Ok(zip_hash.to_string());
                }
                _ => {}
            }
        }
        Err(MgError::Integrity(format!(
            "sumdb lookup response has no record for {module}@{version} (fail-closed)"
        )))
    }

    /// Download the full module artifact set (zip + go.mod + .info) for an
    /// entry — the materializer needs all three for a GOPROXY=off cache.
    /// Tải trọn bộ artifact của một version module (zip + go.mod + .info) —
    /// materializer cần cả ba cho cache GOPROXY=off.
    pub async fn download_module(&self, entry: &ResolvedEntry) -> MgResult<GoModuleFiles> {
        let version = format!("v{}", entry.version.trim_start_matches('v'));
        let gomod = self
            .get_bytes(&self.proxy_url_for(&entry.name, &format!("{version}.mod")))
            .await?;
        let expected_mod_sha256 = entry
            .extra_markers
            .iter()
            .find_map(|marker| marker.strip_prefix(GO_MOD_SHA256_MARKER_PREFIX))
            .ok_or_else(|| {
                MgError::Integrity(format!(
                    "Go resolution did not pin the .mod metadata for {}@{}; run `mgc install` again",
                    entry.name, entry.version
                ))
            })?;
        let actual_mod_sha256 = sha256_hex(&gomod);
        if !actual_mod_sha256.eq_ignore_ascii_case(expected_mod_sha256) {
            return Err(MgError::Integrity(format!(
                "Go .mod metadata changed after resolution for {}@{} (fail-closed)",
                entry.name, entry.version
            )));
        }
        let info = self
            .get_bytes(&self.proxy_url_for(&entry.name, &format!("{version}.info")))
            .await?;
        validate_go_info(&info, &version)?;
        let zip = self.get_bytes(&entry.artifact_url).await?;
        Ok(GoModuleFiles { zip, gomod, info })
    }
}
