//! Source-file resolution owner wiring.
//! Nối owner resolve file source.

#[path = "error.rs"]
mod error;
#[path = "source_file_resolution.rs"]
mod implementation;

pub(super) use self::error::SourceFileResolutionError;
pub(super) use self::implementation::{resolve_project_file, resolve_source_file};
