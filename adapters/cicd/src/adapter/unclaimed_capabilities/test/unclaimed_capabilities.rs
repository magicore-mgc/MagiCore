#![cfg(test)]

//! Keep every unclaimed provider trait implemented with shared defaults.
//! Giữ mọi trait provider chưa claim với default dùng chung.

use crate::adapter::CicdAdapter;

fn requires_unclaimed_traits<
    T: mgc_types::capabilities::DependencyResolver
        + mgc_types::capabilities::ArtifactFetcher
        + mgc_types::capabilities::ContentStoreProvider
        + mgc_types::capabilities::LockfileProvider
        + mgc_types::capabilities::LifecycleRunner
        + mgc_types::capabilities::OptimizerProvider
        + mgc_types::capabilities::Materializer
        + mgc_types::capabilities::SimulatorProvider
        + mgc_types::capabilities::DeviceProvider
        + mgc_types::capabilities::DeployProvider
        + mgc_types::capabilities::ModelRuntimeProvider,
>() {
}

#[test]
fn unsupported_capability_traits_remain_implemented() {
    requires_unclaimed_traits::<CicdAdapter>();
}
