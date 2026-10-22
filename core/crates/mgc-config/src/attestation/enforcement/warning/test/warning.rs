//! Warning-gate unit tests / Test đơn vị cổng cảnh báo.
//! Confirms exactly one caller claims an unset gate.

use super::super::claim_once;
use std::sync::OnceLock;

#[test]
fn warning_gate_can_be_claimed_once() {
    let gate = OnceLock::new();

    assert!(claim_once(&gate));
    assert!(!claim_once(&gate));
}
