//! Stable lockfile parser, bounded-reader, loader and signature API.
//! API ổn định cho parser lockfile, bounded reader, loader và chữ ký.

mod bounded_reader;
mod error;
mod load;
mod parser;
mod signature;
mod verification;

pub use self::bounded_reader::{
    read_bounded_regular_file, read_lockfile_bytes, read_signature_file_bytes,
};
pub use self::load::{ensure_v3_readable_lock, load_lock_document, load_lockfile};
pub use self::parser::{LockDocument, parse_document, parse_lockfile};
#[allow(deprecated)]
pub use self::signature::is_lockfile_signed;
pub use self::signature::{signature_file_presence, signature_path_for};
pub use self::verification::load_and_verify_lockfile;
