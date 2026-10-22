//! Stable SARIF output facade.
//! Facade output SARIF ổn định.

mod driver;
mod result;
mod sarif;
mod types;

pub use self::sarif::to_sarif;
pub use self::types::{
    SARIF_SCHEMA, SARIF_VERSION, SarifDriver, SarifLocation, SarifLog, SarifLogicalLocation,
    SarifMessage, SarifResult, SarifRun, SarifTool,
};

#[cfg(test)]
#[path = "test/sarif.rs"]
mod tests;
