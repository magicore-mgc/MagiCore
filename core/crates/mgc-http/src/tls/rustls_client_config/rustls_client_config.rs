//! Builds rustls trust and client-auth configuration.
//! Dựng cấu hình trust và client-auth cho rustls.

use super::super::certificate_material::{for_each_ca_cert, load_certs, load_private_key};
use crate::tls::TlsConfig;
use anyhow::Result;
use rustls::pki_types::{CertificateDer, ServerName};
use rustls::{
    ClientConfig, DigitallySignedStruct, Error as RustlsError, RootCertStore, SignatureScheme,
    client::danger::{HandshakeSignatureValid, ServerCertVerified},
};
use std::sync::Arc;

impl TlsConfig {
    /// Build rustls::ClientConfig
    /// Dựng cấu hình client cho rustls.
    pub fn build_rustls_config(&self) -> Result<Arc<ClientConfig>> {
        let mut root_store = RootCertStore::empty();

        // Load system roots using rustls-native-certs
        // Nạp root certificate hệ thống bằng rustls-native-certs.
        // 0.8: load_native_certs() returns CertificateResult — certs carry
        // CertificateDer; any load error fails closed.
        // (0.8: load_native_certs() trả CertificateResult — certs mang
        // CertificateDer; mọi lỗi load đều fail-closed.)
        let native_certs = rustls_native_certs::load_native_certs();
        if let Some(err) = native_certs.errors.first() {
            return Err(anyhow::anyhow!("load native certs: {err}"));
        }
        for cert in native_certs.certs {
            root_store.add(cert)?;
        }

        // Add custom CA if provided
        // Thêm CA tùy chỉnh nếu người dùng cung cấp.
        if let Some(ca_path) = &self.ca_bundle {
            for_each_ca_cert(ca_path, |cert| {
                root_store
                    .add(cert)
                    .map_err(|e| anyhow::anyhow!("add custom CA: {}", e))
            })?;
        }

        // Build config with TLS versions
        // Dựng cấu hình với các phiên bản TLS được hỗ trợ.
        let builder = ClientConfig::builder_with_protocol_versions(&[
            &rustls::version::TLS12,
            &rustls::version::TLS13,
        ])
        .with_root_certificates(root_store);

        // Build config with client cert (mTLS) if provided
        // Dùng chứng chỉ client nếu có đủ cặp cert và key.
        let config =
            if let (Some(cert_path), Some(key_path)) = (&self.client_cert, &self.client_key) {
                let certs = load_certs(cert_path)?;
                let key = load_private_key(key_path)?;
                builder
                    .with_client_auth_cert(certs, key)
                    .map_err(|e| anyhow::anyhow!("load client cert/key: {}", e))?
            } else {
                let config = builder.with_no_client_auth();
                if self.allow_untrusted {
                    ClientConfig::builder_with_protocol_versions(&[
                        &rustls::version::TLS12,
                        &rustls::version::TLS13,
                    ])
                    .dangerous()
                    .with_custom_certificate_verifier(Arc::new(NoVerify::new()))
                    .with_no_client_auth()
                } else {
                    config
                }
            };

        Ok(Arc::new(config))
    }
}

/// Certificate verifier that accepts everything (DANGEROUS)
/// Bộ xác minh chấp nhận mọi chứng chỉ, có rủi ro cao.
#[derive(Debug)]
struct NoVerify;

impl NoVerify {
    fn new() -> Self {
        Self
    }
}

impl rustls::client::danger::ServerCertVerifier for NoVerify {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<ServerCertVerified, RustlsError> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, RustlsError> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, RustlsError> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        vec![
            SignatureScheme::RSA_PKCS1_SHA256,
            SignatureScheme::RSA_PKCS1_SHA384,
            SignatureScheme::RSA_PKCS1_SHA512,
            SignatureScheme::ECDSA_NISTP256_SHA256,
            SignatureScheme::ECDSA_NISTP384_SHA384,
            SignatureScheme::ED25519,
        ]
    }
}

#[cfg(test)]
#[path = "test/rustls_client_config.rs"]
mod tests;
