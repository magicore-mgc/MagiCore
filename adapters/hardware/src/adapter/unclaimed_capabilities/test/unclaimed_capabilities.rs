#![cfg(test)]

//! Keep every unclaimed capability on its inherited shared contract.
//! Giữ mọi capability chưa claim theo hợp đồng dùng chung kế thừa.

use crate::adapter::HardwareAdapter;

fn requires_unclaimed_traits<
    T: mgc_types::capabilities::ScaffoldProvider
        + mgc_types::capabilities::DependencyResolver
        + mgc_types::capabilities::ArtifactFetcher
        + mgc_types::capabilities::ContentStoreProvider
        + mgc_types::capabilities::LockfileProvider
        + mgc_types::capabilities::LifecycleRunner
        + mgc_types::capabilities::Materializer
        + mgc_types::capabilities::SimulatorProvider
        + mgc_types::capabilities::DeviceProvider
        + mgc_types::capabilities::DeployProvider
        + mgc_types::capabilities::ModelRuntimeProvider,
>() {
}

#[test]
fn unsupported_capability_traits_remain_implemented() {
    requires_unclaimed_traits::<HardwareAdapter>();
}
