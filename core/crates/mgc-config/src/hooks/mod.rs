//! User hook owners and the stable public API.
//! Các owner hook và API công khai ổn định.

mod command_parser;
mod config;
mod dependency_policy;
mod execution;
mod listing;
mod merge;
mod paths;

pub use config::HooksConfig;
pub use execution::run_hooks;
pub use listing::list_hooks;
pub use paths::hooks_paths;
