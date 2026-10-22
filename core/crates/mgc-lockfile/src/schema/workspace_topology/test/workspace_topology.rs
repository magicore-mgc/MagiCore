//! Covers default and omission behavior for workspace topology.
//! Kiểm tra mặc định và hành vi lược field của topology workspace.

use super::super::WorkspaceTopology;

#[test]
fn empty_topology_defaults_and_serializes_as_an_empty_table() {
    let topology: WorkspaceTopology =
        serde_json::from_value(serde_json::json!({})).expect("test operation must succeed");
    assert!(topology.members.is_empty());
    assert!(topology.cross_core_edges.is_empty());
    assert_eq!(
        serde_json::to_value(topology).expect("test operation must succeed"),
        serde_json::json!({})
    );
}
