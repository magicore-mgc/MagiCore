//! POM property extraction and literal managed versions.
//! Trích properties POM và managed version literal.

mod pom_properties;
pub use pom_properties::{collect_managed_versions, collect_pom_properties};
