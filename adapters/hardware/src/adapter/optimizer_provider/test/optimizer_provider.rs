#![cfg(test)]

//! The hardware probe answers `Unsupported` for the optimizer it cannot run.
//! Probe hardware trả `Unsupported` cho optimizer mà nó không chạy được.

use crate::adapter::HardwareAdapter;
use mgc_types::MgError;
use mgc_types::capabilities::OptimizerProvider;

#[test]
fn optimizer_probe_fails_closed_for_the_template_listing() {
    let error = HardwareAdapter
        .probe_optimizer()
        .expect_err("hardware has no optimizer of its own in v1.2.0");
    match error {
        MgError::Unsupported {
            core,
            capability,
            guidance,
        } => {
            assert_eq!(core, "hardware");
            assert_eq!(capability, "optimizer");
            assert!(
                guidance.contains("template listing"),
                "guidance must name the honest state, got: {guidance}"
            );
        }
        other => panic!("expected MgError::Unsupported, got {other:?}"),
    }
}
