//! Capability split-language inventory wiring — wiring danh sách split-language.
mod split_languages;
pub use split_languages::SPLIT_LANGUAGES;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
