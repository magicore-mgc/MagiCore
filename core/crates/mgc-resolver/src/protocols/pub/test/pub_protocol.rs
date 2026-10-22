//! Public protocol constructor contract tests.
//! Test contract khởi tạo protocol public.

use super::super::PubProtocol;

#[test]
fn explicit_api_url_trims_trailing_slashes() {
    assert_eq!(
        PubProtocol::new("https://pub.dev///").api_url,
        "https://pub.dev"
    );
}
