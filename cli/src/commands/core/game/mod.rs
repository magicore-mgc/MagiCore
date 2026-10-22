//! Game core command support.
//! Hỗ trợ lệnh riêng cho Game core.

#[path = "error/mod.rs"]
mod error;
#[cfg(feature = "game")]
mod manifest_snapshot;
#[cfg(feature = "game")]
mod optimizer_dependency;
#[cfg(feature = "game")]
mod optimizer_destination;
#[cfg(feature = "game")]
mod optimizer_template;
#[cfg(feature = "game")]
mod project_validation;
#[cfg(feature = "game")]
#[path = "staging/mod.rs"]
mod staging;

#[cfg(feature = "game")]
pub use optimizer_template::game_optimizer_template;
#[cfg(feature = "game")]
pub(crate) use project_validation::validate_game_optimizer_project;
