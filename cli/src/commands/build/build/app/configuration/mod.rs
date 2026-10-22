#[path = "error.rs"]
mod error;
#[path = "configuration.rs"]
mod implementation;

#[cfg(all(test, feature = "app"))]
pub(crate) use implementation::resolve_app_language;
pub(in crate::commands::build::implementation) use implementation::{
    read_app_build_config, resolve_app_language_from_config,
};
