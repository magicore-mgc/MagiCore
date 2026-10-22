//! Optimizer configuration loading orchestration.
//! (Điều phối việc nạp cấu hình optimizer.)

mod loader;

pub use loader::load_optimizer_env;

#[cfg(test)]
#[path = "test/loader.rs"]
mod tests;
