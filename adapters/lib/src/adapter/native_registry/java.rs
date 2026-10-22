//! Construct the native Maven POM protocol.
//! Dựng protocol Maven POM native.

use mgc_resolver::protocols::{MavenProtocol, RegistryProtocol};

pub(super) fn create() -> Box<dyn RegistryProtocol> {
    Box::new(MavenProtocol::from_env())
}
