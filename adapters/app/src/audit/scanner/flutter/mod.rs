//! Flutter audit owner wiring.
//! File này chỉ nối audit production và parser fixture.

mod flutter;
#[cfg(test)]
mod outdated_parser;

pub use flutter::{audit_flutter, dependency_health_flutter};
