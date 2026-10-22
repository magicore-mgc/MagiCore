//! Metafile document reader wiring.
//! Wiring bộ đọc document metafile.

mod document;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

pub(super) use document::validate_document;
