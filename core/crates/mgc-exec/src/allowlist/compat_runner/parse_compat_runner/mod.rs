#[path = "parse_compat_runner.rs"]
mod parse_compat_runner;

pub use parse_compat_runner::parse_compat_runner;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
