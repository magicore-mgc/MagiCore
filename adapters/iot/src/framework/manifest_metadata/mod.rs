//! Export crate-internal IoT manifest detection and target lookup.
//! Export hàm nội bộ đọc IoT manifest và target.

mod manifest_metadata;
pub(crate) use manifest_metadata::{manifest_is_iot, target_from_manifest};
