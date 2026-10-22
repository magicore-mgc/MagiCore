//! Single-selection prompt wiring — wiring prompt chọn một.
//! Re-exports the stable select function.
//! Re-export hàm select ổn định.

mod select;
#[cfg(test)]
#[path = "test/select.rs"]
mod tests;

pub use select::select;
