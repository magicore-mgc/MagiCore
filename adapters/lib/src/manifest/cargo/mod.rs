//! Cargo manifest owner / Owner manifest Cargo.
mod cargo;
pub(crate) use self::cargo::{parse_cargo_manifest, write_cargo_manifest};
