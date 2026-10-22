//! Podfile.lock section and local-pod parser tests.
//! Test parser section và pod cục bộ trong Podfile.lock.

use super::*;

#[test]
fn reads_only_root_pods_and_normalizes_checksum_hex() {
    let lock = parse_podfile_lock(
        "PODS:\n  - Root/Sub (1.2.3):\n    - Nested (= 1.2.3)\n  - Local (from `../local`)\n\nSPEC CHECKSUMS:\n  Root: ABCDEF\n",
    );
    assert_eq!(
        lock.pods,
        vec![PodfileLockPod {
            name: "Root".to_string(),
            version: "1.2.3".to_string()
        }]
    );
    assert_eq!(lock.checksum_for("Root"), Some("abcdef"));
}
