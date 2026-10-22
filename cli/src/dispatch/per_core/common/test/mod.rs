// Common adapter test modules — wire tests by command responsibility.
// Module test adapter common — nối test theo trách nhiệm lệnh.
#[path = "basic_commands.rs"]
mod basic_commands;
#[path = "package_maintenance.rs"]
mod package_maintenance;
#[path = "project_configuration.rs"]
mod project_configuration;
#[path = "publish.rs"]
mod publish;
#[path = "registry_model.rs"]
mod registry_model;
#[path = "sbom.rs"]
mod sbom;
#[path = "update_and_release.rs"]
mod update_and_release;
