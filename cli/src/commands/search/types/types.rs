//! Define private data shapes exchanged by search owners.
//! Khai báo cấu trúc dữ liệu riêng trao đổi giữa các owner search.

use serde::Serialize;
use serde_json::Value;

pub(in crate::commands::search) struct RegistrySearchResponse {
    pub(in crate::commands::search) total: u64,
    pub(in crate::commands::search) objects: Vec<Value>,
}

#[derive(Serialize)]
pub(in crate::commands::search) struct SearchOutput {
    pub(in crate::commands::search) total: u64,
    pub(in crate::commands::search) results: Vec<SearchResult>,
}

#[derive(Serialize)]
pub(in crate::commands::search) struct SearchResult {
    pub(in crate::commands::search) name: String,
    pub(in crate::commands::search) version: String,
    pub(in crate::commands::search) description: String,
    pub(in crate::commands::search) score: String,
    pub(in crate::commands::search) core_label: String,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
