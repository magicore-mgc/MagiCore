//! Shared adapter-factory public facade.
//! Facade chung của factory adapter.

mod available_cores;
mod create_adapter;
mod create_adapter_for;
mod create_adapter_for_scoped;
mod project_uses_npm_registry;

pub use available_cores::available_cores;
pub use create_adapter::create_adapter;
pub use create_adapter_for::create_adapter_for;
pub use create_adapter_for_scoped::create_adapter_for_scoped;
pub use project_uses_npm_registry::project_uses_npm_registry;
