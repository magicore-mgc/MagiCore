//! Preserve the three-part target tuple contract.
//! Giữ contract tuple target gồm đúng ba phần.

#![allow(clippy::unwrap_used)]

use super::*;

#[test]
fn parses_and_formats_three_nonempty_parts() {
    let target = TargetTuple::parse("linux-x86_64-gnu").expect("test operation must succeed");
    assert_eq!(target.os, "linux");
    assert_eq!(target.arch, "x86_64");
    assert_eq!(target.abi, "gnu");
    assert_eq!(target.canonical(), "linux-x86_64-gnu");
    assert!(TargetTuple::parse("linux-x86_64").is_none());
    assert!(TargetTuple::parse("linux--gnu").is_none());
    assert!(TargetTuple::parse("linux-x86_64-gnu-extra").is_none());
}
