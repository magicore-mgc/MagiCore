//! reqwest TLS application tests.
//! Test áp dụng TLS cho reqwest.

use super::*;

#[test]
fn default_tls_options_apply_without_network_io() {
    assert!(
        TlsConfig::default()
            .apply(reqwest::Client::builder())
            .is_ok()
    );
}
