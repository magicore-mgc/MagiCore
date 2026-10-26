//! Output scan owner.
//! Owner scan output.

#[path = "scan.rs"]
mod implementation;

pub(crate) use implementation::scan;
#[cfg(test)]
pub(crate) use implementation::{Report, Severity};
