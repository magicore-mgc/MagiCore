//! Wire resolved-graph projection and duplicate-identity tests.
//! Nối projection resolved graph và test identity trùng.

mod resolved_packages;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(super) use self::resolved_packages::resolved_package_entries;
