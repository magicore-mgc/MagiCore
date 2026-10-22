//! Stable App install API for verified Pub archive cache operations.
//! API ổn định của install App cho thao tác cache archive Pub đã xác minh.
use mgc_types::MgResult;
use std::path::Path;

pub(crate) fn load_verified(root: &Path, expected: &str) -> MgResult<Option<Vec<u8>>> {
    super::archive_load::load_verified(root, expected)
}

pub(crate) fn save_verified(root: &Path, expected: &str, bytes: &[u8]) -> MgResult<()> {
    super::archive_save::save_verified(root, expected, bytes)
}
