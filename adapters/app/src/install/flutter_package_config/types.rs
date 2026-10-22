//! Serde models for Dart package_config.json and package_graph.json.
//! Các type serde cho package_config.json và package_graph.json của Dart.

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct FlutterPackageConfig {
    pub(super) config_version: u8,
    pub(super) generator: &'static str,
    pub(super) generator_version: &'static str,
    pub(super) packages: Vec<FlutterPackageConfigEntry>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct FlutterPackageConfigEntry {
    pub(super) name: String,
    pub(super) root_uri: String,
    pub(super) package_uri: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) language_version: Option<String>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct FlutterPackageGraph {
    pub(super) roots: Vec<String>,
    pub(super) packages: Vec<FlutterPackageGraphEntry>,
    pub(super) config_version: u8,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct FlutterPackageGraphEntry {
    pub(super) name: String,
    pub(super) version: String,
    pub(super) dependencies: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) dev_dependencies: Option<Vec<String>>,
}
