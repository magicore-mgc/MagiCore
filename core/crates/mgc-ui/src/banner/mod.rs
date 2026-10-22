//! Banner owner wiring — wiring cho owner banner.
//! Keeps the root function path stable while isolating banner presentation.
//! Giữ nguyên đường dẫn hàm gốc và cô lập phần trình bày banner.

mod banner;
#[cfg(test)]
#[path = "test/banner.rs"]
mod tests;

pub use banner::print_banner;
