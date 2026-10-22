//! Podfile.lock public value compatibility tests.
//! Test tương thích giá trị Podfile.lock công khai.

use super::super::parse_podfile_lock;
use super::*;

#[test]
fn podfile_lock_parse_pods_and_checksums() {
    let text = "PODS:\n  - Alamofire (5.6.4)\n  - Firebase/Core (10.18.0):\n    - FirebaseCore (= 10.18.0)\n  - FirebaseCore (10.18.0)\n  - LocalOnly (from `../local`)\n\nDEPENDENCIES:\n  - Alamofire (~> 5.6)\n  - Firebase/Core\n\nSPEC CHECKSUMS:\n  Alamofire: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n  Firebase: bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\n  FirebaseCore: cccccccccccccccccccccccccccccccccccccccc\n\nPODFILE CHECKSUM: dddddddddddddddddddddddddddddddddddddddd\n";
    let lock = parse_podfile_lock(text);
    let names: Vec<&str> = lock.pods.iter().map(|pod| pod.name.as_str()).collect();
    assert_eq!(names, vec!["Alamofire", "Firebase", "FirebaseCore"]);
    assert_eq!(lock.pods[1].version, "10.18.0");
    assert_eq!(
        lock.checksum_for("Alamofire"),
        Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
    );
    assert!(lock.checksum_for("FirebaseCore").is_some());
}

#[test]
fn pod_root_keeps_only_the_root_spec_name() {
    assert_eq!(pod_root("Firebase/Core"), "Firebase");
    assert_eq!(pod_root("Alamofire"), "Alamofire");
}
