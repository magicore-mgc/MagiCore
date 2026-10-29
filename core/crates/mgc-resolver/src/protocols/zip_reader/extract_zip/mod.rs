//! ZIP extraction facade — wiring cho giải nén ZIP.

mod extract_zip;

pub use extract_zip::{extract_zip, extract_zip_with_policy};
