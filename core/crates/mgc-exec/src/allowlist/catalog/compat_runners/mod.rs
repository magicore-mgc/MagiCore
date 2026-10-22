#[path = "compat_runners.rs"]
mod compat_runners;

pub use compat_runners::QUALIFIED_COMPAT_RUNNERS;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
