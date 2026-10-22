//! Authentication and trusted transport configuration.
// Cấu hình admin/trusted publishing, proxy, loopback và key.

use crate::auth::AuthService;
use crate::trusted::TrustedPublishingConfig;
use axum::http::HeaderMap;
use mgc_crypto::KeyPair;
use std::net::{IpAddr, SocketAddr};

impl AuthService {
    pub fn with_trusted_publishing(
        mut self,
        config: Option<TrustedPublishingConfig>,
        key: Option<KeyPair>,
    ) -> Self {
        self.trusted_config = config;
        self.attestation_key = key;
        self
    }

    /// Permit HTTP trusted-token exchange only when the server binds loopback.
    /// Chỉ cho phép đổi token qua HTTP khi server bind vào loopback.
    pub fn allow_insecure_trusted_http(mut self, allow: bool) -> Self {
        self.allow_insecure_trusted_http = allow;
        self
    }

    /// Trust forwarded HTTPS only from explicitly configured proxy peers.
    /// Chỉ tin HTTPS chuyển tiếp từ địa chỉ peer proxy đã cấu hình cụ thể.
    pub fn trusted_proxy_ips(mut self, addresses: Vec<IpAddr>) -> Self {
        self.trusted_proxy_ips = addresses;
        self
    }

    pub fn trusted_transport_allowed(&self, headers: &HeaderMap, peer: Option<SocketAddr>) -> bool {
        let Some(peer) = peer else {
            return false;
        };
        (self.trusted_proxy_ips.contains(&peer.ip())
            && headers
                .get("x-forwarded-proto")
                .and_then(|value| value.to_str().ok())
                .is_some_and(|value| value.eq_ignore_ascii_case("https")))
            || (self.allow_insecure_trusted_http && peer.ip().is_loopback())
    }

    pub fn trusted_enabled(&self) -> bool {
        self.trusted_config.is_some() && self.attestation_key.is_some()
    }

    pub fn attestation_key(&self) -> Option<&KeyPair> {
        self.attestation_key.as_ref()
    }
}
