//! Hunk parser module exports.
//! Export module phân tích hunk.

mod error;
mod hunk_parser;

pub(super) use hunk_parser::{parse_file_header, parse_hunk};
