//! Project registry and package-index sources.
//! Registry và package index source của project.

#[path = "registry_sources.rs"]
mod registry_sources;

pub use registry_sources::SourceConfig;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
