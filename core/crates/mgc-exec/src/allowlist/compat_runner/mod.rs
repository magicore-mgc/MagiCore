#[path = "compat_runner.rs"]
mod compat_runner;
#[path = "parse_compat_runner/mod.rs"]
mod parse_compat_runner;

pub use compat_runner::compat_runner_allows;
pub use parse_compat_runner::parse_compat_runner;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
