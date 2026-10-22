//! Maven scanner exports and owner wiring.
//! Export và nối các owner scanner Maven.

mod gradle_verification_metadata;
mod maven;
mod pom_gavs;
mod query_maven_pins;

pub use gradle_verification_metadata::read_gradle_verification_metadata;
pub use maven::audit_java;
pub use pom_gavs::read_pom_gavs;
