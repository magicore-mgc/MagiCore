//! Multi-selection prompt wiring — wiring prompt chọn nhiều.
//! Re-exports the stable multi_select function.
//! Re-export hàm multi_select ổn định.

mod multi_select;
#[cfg(test)]
#[path = "test/multi_select.rs"]
mod tests;

pub use multi_select::multi_select;
