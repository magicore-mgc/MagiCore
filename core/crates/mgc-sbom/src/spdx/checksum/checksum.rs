//! SPDX checksum DTO — DTO checksum SPDX

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::spdx) struct SpdxChecksum {
    pub(in crate::spdx) algorithm: &'static str,
    pub(in crate::spdx) checksum_value: String,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
