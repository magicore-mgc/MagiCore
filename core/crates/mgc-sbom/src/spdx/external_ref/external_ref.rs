//! SPDX external reference DTO — DTO external reference SPDX

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::spdx) struct SpdxExternalRef {
    pub(in crate::spdx) reference_category: &'static str,
    pub(in crate::spdx) reference_type: &'static str,
    pub(in crate::spdx) reference_locator: String,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
