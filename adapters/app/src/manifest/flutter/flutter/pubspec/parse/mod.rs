//! Pubspec parsing owner wiring.
//! Wiring owner parse pubspec.

mod parse;
pub(in crate::manifest::flutter::flutter) use parse::parse_pubspec;
