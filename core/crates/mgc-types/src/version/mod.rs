//! Exports the existing semantic-version type through focused owners.
//! Export type semantic-version hiện có qua các owner chuyên trách.

mod compare_prerelease;
mod display;
mod is_post_or_dev_segment;
mod ordering;
mod parse;
mod precedence_cmp;
#[cfg(test)]
#[cfg(test)]
#[path = "test/support/mod.rs"]
mod test_support;
mod version;

pub use self::version::Version;
