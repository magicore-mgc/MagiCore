//! Run integration-test owner wiring.
//! Wiring owner cho integration test executor.

#[path = "test_support/mod.rs"]
mod test_support;

#[path = "execution_contracts/mod.rs"]
mod execution_contracts;
#[path = "go_runtime_probe/mod.rs"]
mod go_runtime_probe;
#[path = "package_manager_policy/mod.rs"]
mod package_manager_policy;
#[path = "process_tree_guard/mod.rs"]
mod process_tree_guard;
#[path = "project_child_lookup/mod.rs"]
mod project_child_lookup;
