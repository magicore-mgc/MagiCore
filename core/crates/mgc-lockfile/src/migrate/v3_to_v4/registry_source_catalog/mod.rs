//! Wires registry source catalog construction and its focused tests.
//! Nối khâu dựng catalog registry và các test chuyên biệt.

mod registry_source_catalog;
pub(super) use self::registry_source_catalog::{build_source_catalog, package_source_id};
#[cfg(test)]
mod test;
