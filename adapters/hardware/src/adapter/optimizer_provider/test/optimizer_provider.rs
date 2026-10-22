#![cfg(test)]

//! The generator-backed optimizer capability remains available.
//! Capability optimizer dựa trên generator vẫn khả dụng.

use crate::adapter::HardwareAdapter;
use mgc_types::capabilities::OptimizerProvider;

#[test]
fn optimizer_probe_succeeds_for_the_mgc_owned_generator() {
    assert!(HardwareAdapter.probe_optimizer().is_ok());
}
