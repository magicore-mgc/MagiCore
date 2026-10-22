//! Wires bounded no-follow lockfile and signature reads.
//! Nối các thao tác đọc lockfile/chữ ký có giới hạn và no-follow.

mod bounded_reader;
mod error;

pub use self::bounded_reader::{
    read_bounded_regular_file, read_lockfile_bytes, read_signature_file_bytes,
};
