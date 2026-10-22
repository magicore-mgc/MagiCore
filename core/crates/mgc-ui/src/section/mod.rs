//! Section-header wiring — wiring cho heading từng bước.
//! Exports the existing crate-root section function.
//! Export hàm section ở crate root như trước.

mod section;
#[cfg(test)]
#[path = "test/section.rs"]
mod tests;

pub use section::section;
