//! Cloud project gate wiring — wiring gate dự án Cloud.
mod cloud_project_gate;
pub use cloud_project_gate::gate_cloud_project;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
