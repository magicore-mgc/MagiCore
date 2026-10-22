//! Coordinates the public Web install command and its focused stages.
//! Điều phối lệnh install Web công khai và các bước chuyên biệt.

mod backend_install;
mod install;
mod monorepo_install;
mod options;
mod plan;
mod preflight;
mod single_target_install;

pub use install::install;
