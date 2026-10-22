//! npm package-lock parser owner — owner parser package-lock npm.

mod npm;
pub(in crate::import) use npm::parse_npm;
