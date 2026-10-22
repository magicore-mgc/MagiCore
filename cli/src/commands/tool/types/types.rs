use serde::{Deserialize, Serialize};

pub(in crate::commands::tool) const TOOL_MANIFEST: &str = ".mgc-tool.json";
pub(in crate::commands::tool) const TOOL_PENDING: &str = ".mgc-tool-installing.json";
pub(in crate::commands::tool) const TOOL_SCHEMA: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::commands::tool) struct ToolRecord {
    pub(in crate::commands::tool) schema: u32,
    pub(in crate::commands::tool) source: String,
    pub(in crate::commands::tool) package: String,
    pub(in crate::commands::tool) selector: String,
    pub(in crate::commands::tool) binary: String,
    pub(in crate::commands::tool) target: String,
    pub(in crate::commands::tool) installed_version: String,
    pub(in crate::commands::tool) project_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::commands::tool) struct PendingToolInstall {
    pub(in crate::commands::tool) schema: u32,
    pub(in crate::commands::tool) source: String,
    pub(in crate::commands::tool) package: String,
    pub(in crate::commands::tool) selector: String,
    pub(in crate::commands::tool) project_name: String,
    pub(in crate::commands::tool) record: Option<ToolRecord>,
}
