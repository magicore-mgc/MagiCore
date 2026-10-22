//! React Native marker compatibility tests.
//! Test tương thích marker React Native.

use super::*;

#[test]
fn lock_marker_prefixes_remain_stable() {
    assert_eq!(PODSPEC_SHA1_MARKER_PREFIX, "podspec-sha1:");
    assert_eq!(RN_TIER_MARKER_PREFIX, "rn-tier:");
}
