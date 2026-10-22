//! Optimizer environment-loader facade.
//! (Facade ổn định cho bộ tải biến môi trường optimizer.)

mod cargo_toml;
mod env_file;
mod env_quote;
mod error;
mod loader;
mod project_files;
mod runtime_files;

#[cfg(test)]
pub(crate) use env_file::parse_env_file;
pub use loader::load_optimizer_env;
