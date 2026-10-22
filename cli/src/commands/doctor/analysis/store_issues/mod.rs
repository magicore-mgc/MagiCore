//! Store and disk diagnostics.
//! Chẩn đoán store và dung lượng đĩa.

mod store_issues;

pub(super) use store_issues::diagnostics;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
