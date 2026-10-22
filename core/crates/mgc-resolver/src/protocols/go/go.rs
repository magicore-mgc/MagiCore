//! Go module proxy native engine (Go modules).
//! Engine native Go module proxy (Go modules).
//!
//! GOPROXY HTTP spec: version list at `{proxy}/{module}/@v/list` (one `vX.Y.Z`
//! per line, pseudo-versions `v0.0.0-…` excluded), `@latest` fallback, and
//! per-version `@v/{version}.info` (JSON `{Version,Time}`), `@v/{version}.mod`
//! (go.mod text) and `@v/{version}.zip` (module zip). Integrity: the proxy's
//! `{version}.ziphash` (hex sha256 of the zip bytes) is the primary check;
//! when the proxy omits it, the checksum database record at
//! `{sum}/lookup/{module}@{version}` is fetched and the zip is verified via
//! the sumdb's `h1:` directory hash (sha256 over sorted per-file
//! `"<file-sha256-hex>  <name>\n"` lines) — the honest equivalent of
//! `golang.org/x/mod/sumdb/dirhash.HashZip`. Fail-closed when neither source
//! can verify.
//! Spec GOPROXY HTTP: danh sách version tại `{proxy}/{module}/@v/list` (mỗi
//! dòng `vX.Y.Z`, bỏ pseudo-version `v0.0.0-…`), fallback `@latest`, và
//! theo version có `@v/{version}.info` (JSON `{Version,Time}`),
//! `@v/{version}.mod` (nội dung go.mod) và `@v/{version}.zip` (zip module).
//! Toàn vẹn: `{version}.ziphash` của proxy (hex sha256 của byte zip) là kiểm
//! tra chính; khi proxy không cung cấp, lấy bản ghi checksum database tại
//! `{sum}/lookup/{module}@{version}` và xác minh zip bằng directory hash
//! `h1:` của sumdb (sha256 trên các dòng `"<file-sha256-hex>  <name>\n"` đã
//! sắp xếp) — tương đương trung thực của
//! `golang.org/x/mod/sumdb/dirhash.HashZip`. Fail-closed khi không nguồn nào
//! xác minh được.

use mgc_http::HttpClient;

const DEFAULT_PROXY_URL: &str = "https://proxy.golang.org";
const DEFAULT_SUM_URL: &str = "https://sum.golang.org";
pub(in crate::protocols::go) const GO_MOD_SHA256_MARKER_PREFIX: &str = "go-mod-sha256:";

/// Native Go module proxy engine.
/// Engine native Go module proxy.
#[derive(Debug, Clone)]
pub struct GoModProtocol {
    pub(in crate::protocols::go) proxy_url: String,
    pub(in crate::protocols::go) sum_url: String,
    pub(in crate::protocols::go) client: HttpClient,
}

/// Downloaded artifacts of one module version (zip + go.mod + .info).
/// Các artifact đã tải của một version module (zip + go.mod + .info).
#[derive(Debug, Clone)]
pub struct GoModuleFiles {
    pub zip: Vec<u8>,
    pub gomod: Vec<u8>,
    pub info: Vec<u8>,
}

impl GoModProtocol {
    /// Build with an explicit proxy URL (default sumdb base).
    /// Dựng với proxy URL tường minh (sumdb base mặc định).
    pub fn new(proxy_url: &str) -> Self {
        Self::with_sum_base(proxy_url, DEFAULT_SUM_URL)
    }

    /// Build with explicit proxy + sumdb base (testability).
    /// Dựng với proxy + sumdb base tường minh (cho test).
    pub fn with_sum_base(proxy_url: &str, sum_url: &str) -> Self {
        Self {
            proxy_url: proxy_url.trim_end_matches('/').to_string(),
            sum_url: sum_url.trim_end_matches('/').to_string(),
            client: mgc_http::HttpClient::default(),
        }
    }

    /// Build from environment: `MGC_GO_PROXY_URL` / `MGC_GO_SUM_URL`.
    /// Dựng từ môi trường: `MGC_GO_PROXY_URL` / `MGC_GO_SUM_URL`.
    pub fn from_env() -> Self {
        let proxy_url = std::env::var("MGC_GO_PROXY_URL")
            .ok()
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| DEFAULT_PROXY_URL.to_string());
        let sum_url = std::env::var("MGC_GO_SUM_URL")
            .ok()
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| DEFAULT_SUM_URL.to_string());
        Self::with_sum_base(&proxy_url, &sum_url)
    }
}

impl Default for GoModProtocol {
    fn default() -> Self {
        Self::from_env()
    }
}
