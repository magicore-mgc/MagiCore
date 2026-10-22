//! Host Node discovery error contract regression.
//! Test hồi quy contract lỗi tìm Node host.

use super::*;

#[test]
fn missing_node_runtime_is_typed_and_keeps_its_message() {
    let error = registry_package_missing_host_node();
    assert_eq!(
        error.to_string(),
        "a host Node.js runtime is required to execute this JavaScript package binary"
    );
    assert!(matches!(
        error.downcast_ref::<NodeRuntimeError>(),
        Some(NodeRuntimeError::HostNodeRuntimeMissing)
    ));
}
