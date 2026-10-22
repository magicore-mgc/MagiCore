//! Adapts shared logical payload measurement to the dedupe-local error owner.
//! Chuyển kết quả đo payload logic dùng chung sang error owner riêng của dedupe.

use std::path::Path;

use super::super::error::VirtualStoreCleanupError;
use crate::commands::safe_filesystem::logical_payload_bytes as measure_payload;

pub(in crate::commands::dedupe::virtual_store_cleanup) fn logical_payload_bytes(
    package_root: &Path,
) -> Result<u64, VirtualStoreCleanupError> {
    Ok(measure_payload(package_root)?)
}
