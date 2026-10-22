//! PlatformIO starter-file owner.
//! Owner file khởi tạo PlatformIO.

mod scaffold_platformio;
pub(in crate::scaffold) use scaffold_platformio::scaffold_platformio;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
