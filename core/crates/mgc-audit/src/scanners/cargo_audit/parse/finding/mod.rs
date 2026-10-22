//! Wire finding projection and local contract tests.
//! Nối ánh xạ finding và test contract cục bộ.

#[path = "finding.rs"]
mod finding;

pub(super) use self::finding::parse_finding;

#[cfg(test)]
#[path = "test/finding.rs"]
mod tests;
