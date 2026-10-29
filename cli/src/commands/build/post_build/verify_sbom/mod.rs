//! verify-sbom owner wiring.
//! Wiring owner verify-sbom.

#[path = "verify_sbom.rs"]
mod implementation;

// The route in `dispatch::common` is the production consumer; the test build
// reaches these items directly, so both cfgs must keep them reachable.
// (Route trong `dispatch::common` là consumer production; build test truy cập
// trực tiếp nên cả hai cfg đều phải giữ item reachable.)
#[allow(unused_imports)]
pub(crate) use implementation::{Verified, signature_path_for, verify};
