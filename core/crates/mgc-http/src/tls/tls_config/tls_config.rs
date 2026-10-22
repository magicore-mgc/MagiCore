//! TLS configuration values, defaults, and environment loading.
//! Giá trị, mặc định và nạp cấu hình TLS từ môi trường.

/// TLS configuration options
/// Tùy chọn cấu hình TLS.
#[derive(Debug, Clone)]
pub struct TlsConfig {
    /// Bypass cert validation (MAGICORE_ALLOW_UNTRUSTED=1) - CHỈ dev/test
    /// Chỉ tắt xác minh chứng chỉ khi phát triển hoặc kiểm thử.
    pub allow_untrusted: bool,
    /// Custom CA bundle path
    /// Đường dẫn tới bộ chứng chỉ CA tùy chỉnh.
    pub ca_bundle: Option<String>,
    /// Client cert (mTLS)
    /// Chứng chỉ client cho xác thực mTLS.
    pub client_cert: Option<String>,
    pub client_key: Option<String>,
    /// Minimum TLS version
    /// Phiên bản TLS tối thiểu.
    pub min_version: TlsVersion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsVersion {
    V1_2,
    V1_3,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            allow_untrusted: false,
            ca_bundle: None,
            client_cert: None,
            client_key: None,
            min_version: TlsVersion::V1_2,
        }
    }
}

impl TlsConfig {
    /// Load từ environment
    pub fn from_env() -> Self {
        Self {
            allow_untrusted: std::env::var("MAGICORE_ALLOW_UNTRUSTED")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false),
            ca_bundle: std::env::var("MAGICORE_CA_BUNDLE").ok(),
            client_cert: std::env::var("MAGICORE_CLIENT_CERT").ok(),
            client_key: std::env::var("MAGICORE_CLIENT_KEY").ok(),
            min_version: TlsVersion::V1_2,
        }
    }
}

#[cfg(test)]
#[path = "test/tls_config.rs"]
mod tests;
