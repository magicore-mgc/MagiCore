//! Board-validated IoT scaffold entry point.
//! Điểm vào scaffold IoT có xác thực board.

mod scaffold_project;
pub use scaffold_project::scaffold_project;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
