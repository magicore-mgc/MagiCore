//! Public registry-command facade.
//! Facade công khai của các lệnh registry.

mod args;
mod registry;
pub(crate) mod registry_url;
mod trusted_binding;
mod user_management;

pub use args::{RegistryArgs, RegistryCmd, UserCmd};
pub use registry::run;
