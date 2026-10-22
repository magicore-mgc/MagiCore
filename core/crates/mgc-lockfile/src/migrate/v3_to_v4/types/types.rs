//! Holds the compact registry catalog shared by v3 conversion owners.
//! Chứa catalog registry gọn dùng chung giữa các owner chuyển đổi v3.

use std::collections::HashMap;

use crate::v4::SourceRef;

/// Source records, URL identities and source-scoped migration warnings.
/// Record nguồn, định danh URL và warning thuộc phạm vi nguồn.
pub(in crate::migrate::v3_to_v4) struct SourceCatalog {
    pub(in crate::migrate::v3_to_v4) sources: Vec<SourceRef>,
    pub(in crate::migrate::v3_to_v4) source_ids: HashMap<String, String>,
    pub(in crate::migrate::v3_to_v4) warnings: Vec<String>,
}
