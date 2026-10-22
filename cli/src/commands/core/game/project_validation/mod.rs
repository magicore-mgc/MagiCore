//! Game optimizer project identity and native manifest checks.
//! Kiểm tra nhận diện project Game và native manifest cho optimizer.
mod project_validation;

pub(super) use project_validation::require_bevy_optimizer_project;
pub(crate) use project_validation::validate_game_optimizer_project;
