//! SPDX creation-info DTO — DTO creation-info SPDX

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::spdx) struct CreationInfo {
    pub(in crate::spdx) creators: Vec<String>,
    pub(in crate::spdx) created: String,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
