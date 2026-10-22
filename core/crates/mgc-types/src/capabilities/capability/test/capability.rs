//! Capability wire contract tests.
//! Test hợp đồng hợp đồng wire capability.

use crate::capabilities::Capability;

#[test]
fn capability_wire_names_order_and_round_trip_are_stable() {
    let expected = [
        (Capability::ProjectDetector, "project-detector"),
        (Capability::ScaffoldProvider, "scaffold-provider"),
        (Capability::DependencyResolver, "dependency-resolver"),
        (Capability::LockfileProvider, "lockfile-provider"),
        (Capability::ArtifactFetcher, "artifact-fetcher"),
        (Capability::ContentStoreProvider, "content-store-provider"),
        (Capability::Materializer, "materializer"),
        (Capability::LifecycleRunner, "lifecycle-runner"),
        (Capability::AuditProvider, "audit-provider"),
        (Capability::OptimizerProvider, "optimizer-provider"),
        (Capability::SimulatorProvider, "simulator-provider"),
        (Capability::DeviceProvider, "device-provider"),
        (Capability::DeployProvider, "deploy-provider"),
        (Capability::ModelRuntimeProvider, "model-runtime-provider"),
    ];
    assert_eq!(
        Capability::ALL,
        expected
            .iter()
            .map(|(cap, _)| *cap)
            .collect::<Vec<_>>()
            .as_slice()
    );
    for (capability, wire_name) in expected {
        assert_eq!(capability.as_str(), wire_name);
        assert_eq!(capability.to_string(), wire_name);
        assert_eq!(
            serde_json::to_string(&capability).unwrap(),
            format!("\"{wire_name}\"")
        );
        assert_eq!(
            serde_json::from_str::<Capability>(&format!("\"{wire_name}\"")).unwrap(),
            capability
        );
    }
}
