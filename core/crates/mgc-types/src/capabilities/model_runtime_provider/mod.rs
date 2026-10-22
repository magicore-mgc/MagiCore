//! Model runtime capability module wiring.
//! Wiring module capability runtime model.

mod model_runtime_provider;
pub use self::model_runtime_provider::ModelRuntimeProvider;
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
