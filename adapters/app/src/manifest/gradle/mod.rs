//! Gradle manifest owner module wiring.
//! Wiring module owner manifest Gradle.

mod catalog_entry;
mod catalog_pin;
mod catalog_reader;
mod dsl_policy;
mod error;
mod gradle;
mod version_ref_uses;

pub use catalog_entry::CatalogEntry;
pub use gradle::{bump_catalog_pin, parse_build_gradle, parse_version_catalog, write_build_gradle};
