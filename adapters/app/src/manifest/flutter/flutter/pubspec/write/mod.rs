//! Pubspec writer owner wiring.
//! Wiring owner writer pubspec.

mod write;
pub(in crate::manifest::flutter::flutter) use write::write_pubspec;
