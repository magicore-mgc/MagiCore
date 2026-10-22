//! Shared adapter contract types and trait façade — type hợp đồng adapter và trait façade dùng chung.

mod add_options;
mod audit_report;
mod dependency_health_report;
mod finding_class;
mod install_options;
mod install_summary;
mod installed_package;
mod manifest_identity;
mod outdated_dependency;
mod package_adapter;
mod prepared_add;
mod resolved_graph;
mod resolved_package;
mod scanner_status;
mod updated_package;
mod vulnerability;
mod vulnerability_severity;

pub use add_options::AddOptions;
pub use audit_report::AuditReport;
pub use dependency_health_report::DependencyHealthReport;
pub use finding_class::FindingClass;
pub use install_options::InstallOptions;
pub use install_summary::{InstallCacheMode, InstallSummary};
pub use installed_package::InstalledPackage;
pub use manifest_identity::ManifestIdentity;
pub use outdated_dependency::OutdatedDependency;
pub use package_adapter::PackageAdapter;
pub use prepared_add::PreparedAdd;
pub use resolved_graph::ResolvedGraph;
pub use resolved_package::ResolvedPackage;
pub use scanner_status::ScannerStatus;
pub use updated_package::UpdatedPackage;
pub use vulnerability::{Vulnerability, now_rfc3339_public};
pub use vulnerability_severity::VulnerabilitySeverity;
