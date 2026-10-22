//! Shared default and named catalog data contract.
//! Hợp đồng dữ liệu catalog mặc định và catalog có tên.

use std::collections::HashMap;

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct WorkspaceCatalogs {
    #[serde(default)]
    pub default: HashMap<String, String>,
    #[serde(flatten)]
    pub named: HashMap<String, HashMap<String, String>>,
}
