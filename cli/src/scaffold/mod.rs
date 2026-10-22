pub mod cache;
mod container;
pub(crate) mod error;
pub mod processor;
pub mod processors;
pub mod provenance;
pub mod registry;
pub mod resolver;
pub mod spec;
pub mod template_root;

pub use processor::Scaffolder;
