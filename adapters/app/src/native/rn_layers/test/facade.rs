#![cfg(test)]

//! Keep the documented public entry paths importable.
//! Giữ các public path đã công bố có thể import.

use crate::native::rn_layers::{RNResolution, install_rn_layers, resolve_rn_layers};

#[test]
fn public_layered_api_paths_are_exported() {
    let _: Option<RNResolution> = None;
    let _ = resolve_rn_layers;
    let _ = install_rn_layers;
}
