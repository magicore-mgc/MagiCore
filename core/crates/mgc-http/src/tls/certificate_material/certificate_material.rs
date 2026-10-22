//! Reads and parses PEM certificate and private-key files.
//! Đọc và phân tích tệp chứng chỉ cùng khóa riêng PEM.

use anyhow::Result;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls_pki_types::pem::PemObject;

/// Parse each custom CA certificate before passing it to the caller.
/// Phân tích từng certificate CA tùy chỉnh trước khi chuyển cho caller.
pub(in crate::tls) fn for_each_ca_cert(
    path: &str,
    mut add_cert: impl FnMut(CertificateDer<'static>) -> Result<()>,
) -> Result<()> {
    let ca_pem =
        std::fs::read_to_string(path).map_err(|e| anyhow::anyhow!("read CA bundle: {}", e))?;
    let mut certificate_count = 0;
    for cert in CertificateDer::pem_slice_iter(ca_pem.as_bytes()) {
        let cert = cert.map_err(|e| anyhow::anyhow!("parse CA cert: {}", e))?;
        add_cert(cert)?;
        certificate_count += 1;
    }
    if certificate_count == 0 {
        anyhow::bail!("parse CA cert: no certificate PEM blocks found");
    }
    Ok(())
}

/// Load certs from PEM file
/// Nạp chứng chỉ từ tệp PEM.
pub(in crate::tls) fn load_certs(path: &str) -> Result<Vec<CertificateDer<'static>>> {
    // PemObject is imported at module scope for consistent PEM parsing.
    // PemObject được import ở scope module để thống nhất cách phân tích PEM.
    let pem = std::fs::read_to_string(path)?;
    let certificates = CertificateDer::pem_slice_iter(pem.as_bytes())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| anyhow::anyhow!("parse certs: {}", e))?;
    if certificates.is_empty() {
        anyhow::bail!("parse certs: no certificate PEM blocks found");
    }
    Ok(certificates)
}

/// Load private key from PEM file
/// Nạp khóa riêng từ tệp PEM.
pub(in crate::tls) fn load_private_key(path: &str) -> Result<PrivateKeyDer<'static>> {
    let pem = std::fs::read_to_string(path)?;
    PrivateKeyDer::from_pem_slice(pem.as_bytes())
        .map_err(|e| anyhow::anyhow!("parse private key: {}", e))
}

#[cfg(test)]
#[path = "test/certificate_material.rs"]
mod tests;
