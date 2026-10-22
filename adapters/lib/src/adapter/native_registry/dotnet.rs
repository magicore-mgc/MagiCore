//! Construct NuGet protocol with the detected consumer target framework.
//! Dựng protocol NuGet với target framework của project.

use crate::adapter::types::LibAdapter;
use mgc_resolver::protocols::{NuGetProtocol, RegistryProtocol};

pub(super) async fn create(adapter: &LibAdapter) -> Box<dyn RegistryProtocol> {
    let mut protocol = NuGetProtocol::from_env().await;
    if let Some(tfm) = adapter.dotnet_tfm.as_deref() {
        protocol = protocol.with_consumer_tfm(tfm);
    }
    Box::new(protocol)
}
