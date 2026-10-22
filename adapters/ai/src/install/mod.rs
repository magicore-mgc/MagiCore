//! AI model-install API wiring and stable exports.
//! Wiring API cài model AI và giữ các đường dẫn export ổn định.

pub mod download;
mod install_model;
mod types;
pub mod verify;

pub use install_model::install_model;
pub use types::InstallSummary;
