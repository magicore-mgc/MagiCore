//! Protocol configuration tests.
//! Test cấu hình protocol.

use super::super::hfhub::HfHubProtocol;

#[test]
fn explicit_base_url_trims_trailing_slashes() {
    let protocol = HfHubProtocol::new("https://hub.example///");
    assert_eq!(protocol.base_url, "https://hub.example");
}
