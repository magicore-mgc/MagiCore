//! Construct the crates.io registry protocol.
//! Dựng protocol registry crates.io.

use mgc_resolver::protocols::{CratesProtocol, RegistryProtocol};

pub(super) fn create() -> Box<dyn RegistryProtocol> {
    Box::new(CratesProtocol::from_env())
}
