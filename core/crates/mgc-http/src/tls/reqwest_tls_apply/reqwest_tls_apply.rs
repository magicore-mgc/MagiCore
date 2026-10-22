//! Applies TLS options to a reqwest client builder.
//! Áp dụng tùy chọn TLS vào client builder của reqwest.

use crate::tls::TlsConfig;
use anyhow::Result;
use reqwest::ClientBuilder;

impl TlsConfig {
    /// Apply TLS options to a reqwest ClientBuilder.
    /// Áp dụng tùy chọn TLS vào reqwest::ClientBuilder.
    pub fn apply(&self, mut builder: ClientBuilder) -> Result<ClientBuilder> {
        if self.allow_untrusted {
            // Loud once-per-process: disabling certificate verification is
            // an explicit operator opt-in (MAGICORE_ALLOW_UNTRUSTED=1), but
            // it must never pass silently — a leaked env var would
            // otherwise downgrade every TLS connection without a trace.
            // (Cảnh báo rõ khi tắt verify cert — env rò rỉ không được hạ
            // TLS âm thầm.)
            static WARNED: std::sync::OnceLock<()> = std::sync::OnceLock::new();
            if WARNED.set(()).is_ok() {
                eprintln!(concat!(
                    "WARNING: TLS certificate verification is DISABLED ",
                    "(MAGICORE_ALLOW_UNTRUSTED=1) — connections are vulnerable to interception"
                ));
            }
            builder = builder.danger_accept_invalid_certs(true);
        }

        if let Some(ca) = &self.ca_bundle {
            let ca_pem =
                std::fs::read(ca).map_err(|e| anyhow::anyhow!("read CA: {}: {}", ca, e))?;
            builder = builder.add_root_certificate(reqwest::Certificate::from_pem(&ca_pem)?);
        }

        if let (Some(cert), Some(key)) = (&self.client_cert, &self.client_key) {
            let cert_pem = std::fs::read(cert)
                .map_err(|e| anyhow::anyhow!("read client cert: {}: {}", cert, e))?;
            let key_pem = std::fs::read(key)
                .map_err(|e| anyhow::anyhow!("read client key: {}: {}", key, e))?;
            // Combine cert and key into a single PEM for reqwest::Identity::from_pem
            // Ghép cert và key thành PEM cho reqwest::Identity::from_pem.
            let combined_pem = [cert_pem.as_slice(), key_pem.as_slice()].concat();
            let identity = reqwest::Identity::from_pem(&combined_pem)
                .map_err(|e| anyhow::anyhow!("create identity: {}", e))?;
            builder = builder.identity(identity);
        }

        Ok(builder)
    }
}

#[cfg(test)]
#[path = "test/reqwest_tls_apply.rs"]
mod tests;
