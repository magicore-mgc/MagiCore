//! Cloud runtime owner exports and nearest tests.
mod cloud;
pub(in crate::commands::optimizer) use cloud::detect_cloud_node_runtime;

#[cfg(test)]
#[path = "test/cloud.rs"]
mod tests;
