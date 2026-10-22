//! Shared Manifest module exports and nearest tests.
//! Export module Manifest dùng chung và test gần owner.

mod manifest;

pub use self::manifest::Manifest;

#[cfg(test)]
#[path = "test/manifest.rs"]
mod tests;
