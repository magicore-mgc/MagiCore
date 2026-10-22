//! Export public SARIF data-transfer types.
//! Export các kiểu dữ liệu SARIF public.

#[path = "types.rs"]
mod types;

pub use self::types::{
    SARIF_SCHEMA, SARIF_VERSION, SarifDriver, SarifLocation, SarifLog, SarifLogicalLocation,
    SarifMessage, SarifResult, SarifRun, SarifTool,
};
