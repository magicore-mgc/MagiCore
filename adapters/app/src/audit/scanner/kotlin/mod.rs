//! Kotlin audit owner wiring.
//! File này chỉ nối audit native và parser regression fixture.

mod kotlin;
#[cfg(test)]
mod owasp_parser;

pub use kotlin::audit_kotlin;
