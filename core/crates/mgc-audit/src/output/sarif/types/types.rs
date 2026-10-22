//! SARIF 2.1.0 serialized model and schema constants.
//! Model SARIF 2.1.0 và hằng schema được serialize.

use serde::Serialize;

pub const SARIF_SCHEMA: &str = "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json";
pub const SARIF_VERSION: &str = "2.1.0";

#[derive(Serialize)]
pub struct SarifLog {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub version: String,
    pub runs: Vec<SarifRun>,
}

#[derive(Serialize)]
pub struct SarifRun {
    pub tool: SarifTool,
    pub results: Vec<SarifResult>,
}

#[derive(Serialize)]
pub struct SarifTool {
    pub driver: SarifDriver,
}

#[derive(Serialize)]
pub struct SarifDriver {
    pub name: String,
    pub version: String,
    pub information_uri: String,
}

#[derive(Serialize)]
pub struct SarifResult {
    /// SARIF level: error for Critical/High, warning for Medium/Low, note
    /// for Info — maps cleanly onto GitHub Security categories.
    /// Cấp SARIF: error cho Critical/High, warning Medium/Low, note Info.
    pub level: &'static str,
    pub message: SarifMessage,
    pub rule_id: String,
    /// Logical location = the affected package (ecosystem:name@version).
    /// Vị trí logic = package dính lỗi (ecosystem:name@version).
    pub locations: Vec<SarifLocation>,
}

#[derive(Serialize)]
pub struct SarifMessage {
    pub text: String,
}

#[derive(Serialize)]
pub struct SarifLocation {
    pub logical_locations: Vec<SarifLogicalLocation>,
}

#[derive(Serialize)]
pub struct SarifLogicalLocation {
    pub name: String,
}
