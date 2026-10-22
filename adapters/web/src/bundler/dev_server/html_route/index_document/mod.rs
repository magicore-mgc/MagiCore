//! HTML index-document owner wiring.
//! Wiring owner cho tài liệu HTML index.

#[path = "index_document.rs"]
mod implementation;

pub(in crate::bundler::dev_server::implementation) use implementation::serve_index;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
