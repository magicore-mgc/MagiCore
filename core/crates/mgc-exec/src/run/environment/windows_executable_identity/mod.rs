//! Windows executable identity owner wiring.
//! Wiring owner định danh executable Windows.

mod windows_executable_identity;

pub(super) use windows_executable_identity::{
    windows_current_executable_sha256, windows_sha256_file,
};
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
