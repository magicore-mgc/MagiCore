#![cfg(test)]

//! Framework aliases map to one stable canonical identifier.
//! Alias framework được ánh xạ về một định danh canonical ổn định.

use crate::framework::framework::canonical_framework;

#[test]
fn zephyr_arm_alias_uses_zephyr_canonical_id() {
    assert_eq!(canonical_framework("zephyr-arm"), Some("zephyr"));
    assert_eq!(canonical_framework("unknown"), None);
}
