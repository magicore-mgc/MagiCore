//! Wiring for platform-specific package executable links.
//! Wiring cho link executable theo từng nền tảng.

mod link;
pub use link::create_bin_link;
