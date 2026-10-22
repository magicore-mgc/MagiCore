//! CocoaPods CDN path-segment compatibility tests.
//! Test tương thích segment URL CDN CocoaPods.

use super::*;

#[test]
fn cocoapods_spec_url_segments() {
    assert_eq!(
        cocoapods_spec_url("https://cdn.cocoapods.org", "Alamofire", "5.6.4"),
        "https://cdn.cocoapods.org/a/Alamofire/5.6.4/Alamofire.podspec.json"
    );
    assert_eq!(
        cocoapods_spec_url("https://cdn.cocoapods.org/", "Bolts", "1.9.0"),
        "https://cdn.cocoapods.org/b/Bolts/1.9.0/Bolts.podspec.json"
    );
    assert_eq!(
        cocoapods_spec_url("https://cdn.cocoapods.org", "2fa", "1.0.0"),
        "https://cdn.cocoapods.org/0-9/2fa/1.0.0/2fa.podspec.json"
    );
}
