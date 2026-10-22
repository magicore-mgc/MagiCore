//! Construct the Go module proxy protocol.
//! Dựng protocol Go module proxy.

use mgc_resolver::protocols::{GoModProtocol, RegistryProtocol};

pub(super) fn create() -> Box<dyn RegistryProtocol> {
    Box::new(GoModProtocol::from_env())
}
