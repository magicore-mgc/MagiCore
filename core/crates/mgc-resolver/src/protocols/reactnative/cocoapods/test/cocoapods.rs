//! CocoaPods protocol construction tests.
//! Test khởi tạo protocol CocoaPods.

use super::*;

#[test]
fn explicit_cdn_trims_trailing_slashes() {
    assert_eq!(
        CocoaPodsProtocol::with_cdn("https://cdn.example///").cdn(),
        "https://cdn.example"
    );
}
