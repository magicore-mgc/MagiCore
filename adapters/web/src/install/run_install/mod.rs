//! Stable install entry points and ordered phase owners.
//! Entry point install ổn định và các owner phase theo thứ tự.

mod commit;
mod fast_path;
mod fetch_preparation;
mod lifecycle;
mod materialization;
mod package_plan;
mod run_install;
mod setup;
mod state;

pub use run_install::{run_install, run_install_with_registry};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
