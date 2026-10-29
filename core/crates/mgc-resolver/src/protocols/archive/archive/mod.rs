//! Wires the stable archive API facade and shared extraction limits.
//! Nối facade API archive ổn định và các giới hạn giải nén dùng chung.

mod archive;
pub(super) use self::archive::{
    MAX_TAR_ENTRIES, MAX_TAR_ENTRY_UNCOMPRESSED, MAX_TAR_TOTAL_UNCOMPRESSED,
};
pub use self::archive::{cargo_source_file_hashes, extract_crate_tar_gz, extract_tar_gz};
pub(crate) use self::archive::{
    ensure_extract_file_target, ensure_extract_root, write_extracted_file,
};
