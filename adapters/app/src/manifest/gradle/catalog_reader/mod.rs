//! Version-catalog reader module wiring.
//! Wiring module đọc version catalog.

mod catalog_reader;

pub(super) use catalog_reader::{parse_catalog_content, parse_version_catalog};
