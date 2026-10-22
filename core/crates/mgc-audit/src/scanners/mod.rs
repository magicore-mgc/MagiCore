//! Shared scanner facade for MGC-owned ecosystem audits.
//! Facade chung cho các scanner audit hệ sinh thái do MGC sở hữu.

pub mod bun_deno;
mod cargo_audit;
pub mod dart;
pub mod dotnet;
pub mod gh_actions;
pub mod govulncheck;
mod input;
pub mod maven;
pub mod osv;
mod pip_audit;
mod python;
pub mod rust;
pub mod terraform;
mod text;

pub use bun_deno::{BunDenoRead, NpmPin, read_bun_lock, read_deno_lock, read_js_lockfiles};
pub(crate) use cargo_audit::cvss_base_severity_for_go;
pub use cargo_audit::{CargoAuditParse, parse_cargo_audit_json};
pub use dart::{audit_flutter_osv, read_pubspec_lock};
pub use dotnet::{audit_dotnet, collect_dotnet_pins, read_packages_lock, read_solution_projects};
pub use gh_actions::{WorkflowFinding, audit_github_actions, scan_workflow_text};
pub use govulncheck::{audit_go, parse_govulncheck_json, read_go_mod_requires};
pub(crate) use input::read_bounded_project_input;
pub use maven::{audit_java, read_gradle_verification_metadata, read_pom_gavs};
pub use osv::{
    OsvPin, audit_cocoapods_osv, audit_osv_pins, audit_swift_spam, read_podfile_lock,
    read_swift_resolved, swift_resolved_path,
};
pub use pip_audit::{PipAuditParse, parse_pip_audit_json};
pub use python::{
    audit_python, find_pylock_file, find_requirements_file, python_pins_from_mgc_lock,
    read_requirements_pins, read_uv_lock_pins,
};
pub use rust::{audit_rust, read_cargo_lock_pins};
pub use terraform::{audit_terraform_lock, parse_terraform_lock};
pub use text::truncate_utf8;
