//! Library runtime owner exports and nearest tests.
mod lib;
pub(in crate::commands::optimizer) use lib::detect_lib_runtime;

#[cfg(test)]
#[path = "test/lib.rs"]
mod tests;
