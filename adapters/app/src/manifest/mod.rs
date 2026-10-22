//! App manifest format owners and stable parse/write exports.
//! Owner manifest App và export parse/write ổn định.

pub mod flutter;
pub mod gradle;
pub mod react_native;
pub mod swift;

mod manifest;
mod parse_manifest;
mod write_manifest;

pub use manifest::{parse_manifest, write_manifest};
