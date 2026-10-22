//! Stable tarball API facade — giữ route tarball công khai ổn định.
//! Stable tarball API facade — preserves the public tarball route.

mod digest;
mod entry_writer;
mod error;
mod pack;
mod staging;
mod types;

pub use pack::pack;
pub use types::PackResult;

use digest::digest_archive;
use entry_writer::append_entries;
use error::PackError;
use staging::{create_staged_output, publish_staged_output};
