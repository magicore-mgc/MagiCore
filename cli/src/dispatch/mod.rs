//! dispatch — run() entry + phân tuyến: engine → common / per_core (v5).

mod add_mapping;
mod bare;
mod common;
mod engine;
mod install_mapping;
mod list_mapping;
mod per_core;
mod remove_mapping;
mod types;
mod update_mapping;

mod core;

pub use engine::run;
