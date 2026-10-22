//! Missing-tool diagnostics.
//! Chẩn đoán thiếu tool.

mod toolchain_issues;

pub(super) use toolchain_issues::diagnostics;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
