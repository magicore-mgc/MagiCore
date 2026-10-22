//! Exact-pin contract checks for Go version selection.
//! Kiểm tra hợp đồng exact-pin của bộ chọn version Go.

use super::super::*;

#[test]
fn exact_pin_version_accepts_bare_and_pseudo() {
    assert_eq!(
        GoModProtocol::exact_pin_version("1.2.3").as_deref(),
        Some("1.2.3")
    );
    assert_eq!(
        GoModProtocol::exact_pin_version("v0.0.0-20161208181325-20d25e280405").as_deref(),
        Some("0.0.0-20161208181325-20d25e280405")
    );
    assert_eq!(
        GoModProtocol::exact_pin_version("=2.0.0").as_deref(),
        Some("2.0.0")
    );
    assert!(GoModProtocol::exact_pin_version("^1.2.3").is_none());
    assert!(GoModProtocol::exact_pin_version(">=1.0.0").is_none());
    assert!(GoModProtocol::exact_pin_version("*").is_none());
    assert!(GoModProtocol::exact_pin_version("").is_none());
}

#[test]
fn exact_pin_rejects_compound_or_whitespace_ranges() {
    for range in ["1.2.3,2.0.0", "1.2.3 || 2.0.0", "= 1.2.3", "~1.2.3"] {
        assert!(GoModProtocol::exact_pin_version(range).is_none(), "{range}");
    }
}
