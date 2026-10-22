//! Metafile input-path validation wiring.
//! Wiring kiểm tra path input metafile.

mod input_path;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

pub(super) use input_path::validate_input;
