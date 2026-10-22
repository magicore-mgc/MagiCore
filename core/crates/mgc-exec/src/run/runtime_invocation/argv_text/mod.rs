//! Strict UTF-8 argv access exports.
//! Export đọc argv UTF-8 nghiêm ngặt.

#[path = "argv_text.rs"]
mod implementation;
pub(in crate::run::implementation) use implementation::argv_text;
