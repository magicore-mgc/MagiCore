//! Go module graph test owner facade.
//! Facade owner test go module graph theo hành vi.

#[path = "go_module_graph.rs"]
mod implementation;

pub(crate) use implementation::resolve_with_protocol_builds_go_module_graph_and_v3_lock_entries;
