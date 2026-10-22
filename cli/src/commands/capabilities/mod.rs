//! Shared capability command facade — facade lệnh capability dùng chung.
//! This module preserves the existing CLI and MCP API paths.
//! Module này giữ nguyên đường API hiện có của CLI và MCP.

mod capabilities;
mod core_catalog;
mod dependency_ownership;
mod error;

pub use capabilities::{json_payload, run};
