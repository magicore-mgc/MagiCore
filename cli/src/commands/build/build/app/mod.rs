#[path = "configuration/mod.rs"]
mod configuration;
#[path = "app.rs"]
mod implementation;
#[path = "multi/mod.rs"]
mod multi;

#[cfg(all(test, feature = "app"))]
pub(crate) use configuration::resolve_app_language;
pub(super) use implementation::build_app;
#[cfg(all(test, feature = "app"))]
pub(crate) use multi::{build_multi_app, finish_multi_build, resolve_multi_platforms};
