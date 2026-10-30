//! Scaffold remedy guidance owner wiring.
//! Wiring owner hướng xử lý scaffold.

#[path = "remedy.rs"]
mod implementation;

// The two descriptive constants document the guidance; the message builder
// already encodes them, so a build that never reads them directly is expected.
// (Hai hằng mô tả tài liệu hoá hướng dẫn; builder message đã dùng chúng nên build
// không đọc trực tiếp là bình thường.)
pub use implementation::missing_layer_next_steps;
#[cfg(test)]
#[allow(unused_imports)] // only the sibling test files read the detector
pub(crate) use implementation::names_nonexistent_subcommand;
#[allow(unused_imports)]
pub use implementation::{OFFLINE_GENERATOR_CORES, REGISTRY_CONFIG_SOURCES};
