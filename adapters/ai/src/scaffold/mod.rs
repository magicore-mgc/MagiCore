//! Offline AI project scaffolding (file writers only, no toolchain spawn).
//! Scaffold project AI offline (chỉ ghi file, không spawn toolchain).

mod layout;
mod mcp_server;
mod project;
mod python_agent;
mod slug;

pub use project::scaffold_project;

#[cfg(test)]
#[path = "test/mod_test.rs"]
mod tests;
