//! TLS defaults unit tests.
//! Test unit cho mặc định TLS.

use super::*;

#[test]
fn tls_config_defaults() {
    let cfg = TlsConfig::default();
    assert!(!cfg.allow_untrusted);
    assert_eq!(cfg.min_version, TlsVersion::V1_2);
}
