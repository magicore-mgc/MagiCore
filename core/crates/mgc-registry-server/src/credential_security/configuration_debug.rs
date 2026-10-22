//! Secret-redacted registry configuration formatting.
//! Format cấu hình registry có che secret.

impl std::fmt::Debug for crate::RegistryServerConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RegistryServerConfig")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("store_dir", &self.store_dir)
            .field(
                "admin_token",
                &self.admin_token.as_ref().map(|_| "[REDACTED]"),
            )
            .field("max_body_size", &self.max_body_size)
            .field("rate_limit_rps", &self.rate_limit_rps)
            .field("upstream", &self.upstream.as_ref().map(|_| "[CONFIGURED]"))
            .field("storage", &self.storage.as_ref().map(|_| "[CONFIGURED]"))
            .field("oidc_issuer_count", &self.oidc_issuers.len())
            .field("oidc_audience", &self.oidc_audience)
            .field("trusted_proxy_ips", &self.trusted_proxy_ips)
            .finish()
    }
}
