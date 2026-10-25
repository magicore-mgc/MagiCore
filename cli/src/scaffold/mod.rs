pub mod cache;
mod container;
pub mod core_initializers;
pub(crate) mod error;
pub mod framework_registry;
pub(crate) mod hardware_marker;
pub mod local_projects;
pub mod processor;
pub mod provenance;
pub mod registry;
pub mod resolver;
pub mod spec;
pub mod template_root;

pub use processor::Scaffolder;
