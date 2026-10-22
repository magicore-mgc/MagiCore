//! Verified-shim module wiring.
//! Wiring module xác minh shim.

mod verified_shim;
pub(in crate::run::implementation::process_scan::monitor::implementation) use verified_shim::is_verified_shadow_npm_probe_shim;
