//! Stable cargo-audit JSON parser facade and nearest-owner test entry.
//! Facade parser JSON cargo-audit ổn định và điểm nối test gần owner.

mod consistency;
mod document;
mod error;
mod finding;
mod parse;
mod severity;

pub use self::parse::parse_cargo_audit_json;

#[cfg(test)]
#[path = "test/parse.rs"]
mod tests;
