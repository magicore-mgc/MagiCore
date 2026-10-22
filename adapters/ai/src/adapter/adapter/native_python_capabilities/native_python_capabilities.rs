//! Wires private Python capability implementations for the AI adapter.
//! Nối các implementation capability Python private của adapter AI.

#[path = "artifact_fetcher/mod.rs"]
mod artifact_fetcher;
#[path = "content_store_provider/mod.rs"]
mod content_store_provider;
#[path = "dependency_resolver/mod.rs"]
mod dependency_resolver;
#[path = "lockfile_provider/mod.rs"]
mod lockfile_provider;
