//! Filesystem tree verification against an extracted-package marker.
//! Xác minh cây filesystem theo marker package đã extract.

mod extracted_content;

pub use extracted_content::extracted_content_matches;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
