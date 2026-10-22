//! Pubspec parser, writer, dependency conversion and data type wiring.
//! Wiring parse, writer, chuyển dependency và type pubspec.

mod dependency;
mod parse;
mod types;
mod write;

pub(super) use parse::parse_pubspec;
pub(super) use write::write_pubspec;
