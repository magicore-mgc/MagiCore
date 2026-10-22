//! Zephyr starter-file owner.
//! Owner file khởi tạo Zephyr.

mod scaffold_zephyr;
pub(in crate::scaffold) use scaffold_zephyr::scaffold_zephyr;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
