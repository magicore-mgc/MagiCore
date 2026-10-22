//! SPDX relationship DTO — DTO relationship SPDX

use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub(in crate::spdx) struct SpdxRelationship {
    pub(in crate::spdx) spdx_element_id: String,
    pub(in crate::spdx) relationship_type: &'static str,
    pub(in crate::spdx) related_spdx_element: String,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
